pub fn configure(app: &mut whisker_config::Config) {
    app.name("Todo")
        .bundle_id("com.konojunya.todoappwhisker")
        .version("0.1.0")
        .build_number(1);

    app.android(|android| {
        android
            .package("com.konojunya.todoappwhisker")
            .application_id("com.konojunya.todoappwhisker")
            .launcher_activity(".MainActivity")
            .min_sdk(24)
            .target_sdk(34);
    });

    app.ios(|ios| {
        ios.bundle_id("com.konojunya.todoappwhisker")
            .scheme("Todo")
            .deployment_target("13.0");
    });
}
