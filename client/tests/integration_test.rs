use testcontainers::{core::WaitFor, images::generic::GenericImage, *};

#[tokio::test(flavor = "multi_thread")]
async fn simple_tangerine_integration_test() {
    let docker = clients::Cli::default();
    let msg = WaitFor::message_on_stdout("server is ready");

    let generic = GenericImage::new("simple_tangerine_server", "latest").with_wait_for(msg.clone());

    let node = docker.run(generic);
    let port = node.get_host_port_ipv4(80);

    assert_eq!(
        "client",
        reqwest::get(format!("http://127.0.0.1:{port}/client/client"))
            .await
            .unwrap()
            .text()
            .await
            .unwrap()
    );
}
