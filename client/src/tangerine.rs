use crate::command::ExecutableCommand;
use crate::command::ShellCommand;
use crate::crypto::Crypto;
use crate::Res;
use aes_gcm::aead::Aead;
use aes_gcm::Nonce;
use bytes::Bytes;
use reqwest::Response;

pub struct Tangerine {
    current_etag: String,
    executables: Option<Vec<Box<dyn ExecutableCommand>>>,
}

impl Tangerine {
    pub fn new() -> Self {
        Tangerine {
            current_etag: String::from(""),
            executables: None,
        }
    }
    pub async fn execute(&self) -> Res<()> {
        if let Some(commands) = &self.executables {
            for command in commands {
                command.execute().unwrap();
            }
        }
        Ok(())
    }

    pub async fn read_http(
        &mut self,
        base_uri: &str,
        client_id: &str,
        crypto: &Crypto<'_>,
    ) -> Res<()> {
        let response = Tangerine::http_execute(base_uri, client_id).await?;

        // Check e_tag
        let e_tag = String::from(
            response
                .headers()
                .get("etag")
                .map(|hval| hval.to_str().unwrap())
                .unwrap_or_else(|| ""),
        );
        if self.current_etag == e_tag {
            println!("Skipping, e_tag is the same as before");
            // This is hacky
            self.executables = None;
            return Ok(());
        }
        println!("Not skipping, new e_tag");
        self.current_etag = e_tag;

        let commands: Vec<Box<dyn ExecutableCommand>> =
            Tangerine::read_tangerine(response.bytes().await?, crypto)?
                .into_iter()
                .map(|command| Box::new(ShellCommand::new(command)) as Box<dyn ExecutableCommand>)
                .collect();
        self.executables = Some(commands);
        Ok(())
    }

    async fn http_execute(base_uri: &str, client_id: &str) -> Res<Response> {
        let response = reqwest::get(format!("{}/{}.tangerine", base_uri, client_id)).await?;
        Ok(response)
    }

    fn read_tangerine(bytes: Bytes, crypto: &Crypto<'_>) -> Res<Vec<String>> {
        let mut lines = bytes.split(|byte| byte == &b'\n');

        let nonce = Nonce::from_slice(crypto.nonce().as_bytes());

        // Skip header
        let header = lines.next().unwrap();
        if header != b"!TANGERINE_ENC" {
            eprintln!("Expected tangerine header");
        }

        let mut decrypted_lines = vec![];

        for line in lines {
            if line == b"!TANGERINE_ENC" {
                continue;
            }
            let decrypted = crypto.cipher().decrypt(nonce, line).unwrap();
            decrypted_lines.push(String::from_utf8(decrypted).unwrap());
        }
        Ok(decrypted_lines)
    }
}
