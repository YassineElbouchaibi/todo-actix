use consulrs::api::check::common::AgentServiceCheckBuilder;
use consulrs::api::service::requests::RegisterServiceRequest;
use consulrs::client::{ConsulClient, ConsulClientSettingsBuilder};
use consulrs::service;

pub trait ConfigureConsulParameters {
    fn get_name(&self) -> String;
    fn get_host(&self) -> String;
    fn get_port(&self) -> u16;
    fn get_health_check_host(&self) -> String;
    fn get_health_check_port(&self) -> u16;
    fn get_health_check_path(&self) -> String;
    fn get_health_check_interval(&self) -> String;
}

pub async fn register_with_consul<T: ConfigureConsulParameters>(
    params: &T,
    address_to_register: String,
    port_to_register: u16,
) -> tokio::task::JoinHandle<()> {
    let consul_host = params.get_host();
    let consul_port = params.get_port();
    let app_registration_name = params.get_name();
    let app_host = address_to_register;
    let app_port = port_to_register;
    let app_health_check_host = params.get_health_check_host();
    let app_health_check_port = params.get_health_check_port();
    let app_health_check_path = params.get_health_check_path();
    let app_health_check_interval = params.get_health_check_interval();

    tokio::spawn(async move {
        // Create a client
        let client = ConsulClient::new(
            ConsulClientSettingsBuilder::default()
                .address(format!("http://{}:{}", consul_host, consul_port,))
                .build()
                .unwrap(),
        )
        .unwrap();

        // Create a service with a health check that queries the
        // service via HTTP every X seconds.
        service::register(
            &client,
            &app_registration_name,
            Some(
                RegisterServiceRequest::builder()
                    .address(app_host)
                    .port(app_port)
                    .check(
                        AgentServiceCheckBuilder::default()
                            .name("health_check")
                            .interval(app_health_check_interval)
                            .http(format!(
                                "http://{}:{}/{}",
                                app_health_check_host, app_health_check_port, app_health_check_path
                            ))
                            .status("passing")
                            .build()
                            .unwrap(),
                    ),
            ),
        )
        .await
        .unwrap();
    })
}
