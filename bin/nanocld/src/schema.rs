// @generated automatically by Diesel CLI.

diesel::table! {
    cargo_configs (key) {
        key -> Uuid,
        created_at -> Timestamptz,
        cargo_key -> Varchar,
        version -> Varchar,
        config -> Jsonb,
    }
}

diesel::table! {
    cargoes (key) {
        key -> Varchar,
        created_at -> Timestamptz,
        name -> Varchar,
        config_key -> Uuid,
        namespace_name -> Varchar,
    }
}

diesel::table! {
    http_metrics (key) {
        key -> Uuid,
        created_at -> Timestamptz,
        expire_at -> Timestamptz,
        date_gmt -> Timestamptz,
        status -> Int8,
        bytes_sent -> Int8,
        content_length -> Int8,
        body_bytes_sent -> Int8,
        request_time -> Float8,
        node_name -> Varchar,
        uri -> Varchar,
        host -> Varchar,
        remote_addr -> Varchar,
        realip_remote_addr -> Varchar,
        server_protocol -> Varchar,
        request_method -> Varchar,
        proxy_host -> Nullable<Varchar>,
        upstream_addr -> Nullable<Varchar>,
        query_string -> Nullable<Varchar>,
        request_body -> Nullable<Varchar>,
        content_type -> Nullable<Varchar>,
        http_user_agent -> Nullable<Varchar>,
        http_referrer -> Nullable<Varchar>,
        http_accept_language -> Nullable<Varchar>,
    }
}

diesel::table! {
    metrics (key) {
        key -> Uuid,
        created_at -> Timestamptz,
        expire_at -> Timestamptz,
        node_name -> Varchar,
        kind -> Varchar,
        data -> Jsonb,
    }
}

diesel::table! {
    namespaces (name) {
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    node_group_links (rowid) {
        node_name -> Varchar,
        node_group_name -> Varchar,
        rowid -> Int8,
    }
}

diesel::table! {
    node_groups (name) {
        name -> Varchar,
    }
}

diesel::table! {
    nodes (name) {
        name -> Varchar,
        ip_address -> Varchar,
    }
}

diesel::table! {
    resource_configs (key) {
        key -> Uuid,
        created_at -> Timestamptz,
        resource_key -> Varchar,
        version -> Varchar,
        data -> Jsonb,
    }
}

diesel::table! {
    resource_kind_versions (resource_kind_name, version) {
        resource_kind_name -> Varchar,
        created_at -> Timestamptz,
        version -> Varchar,
        schema -> Jsonb,
    }
}

diesel::table! {
    resource_kinds (name) {
        name -> Varchar,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    resources (key) {
        key -> Varchar,
        created_at -> Timestamptz,
        kind -> Varchar,
        config_key -> Uuid,
    }
}

diesel::table! {
    vm_configs (key) {
        key -> Uuid,
        created_at -> Timestamptz,
        vm_key -> Varchar,
        version -> Varchar,
        config -> Jsonb,
    }
}

diesel::table! {
    vm_images (name) {
        name -> Varchar,
        created_at -> Timestamptz,
        kind -> Varchar,
        path -> Varchar,
        format -> Varchar,
        size_actual -> Int8,
        size_virtual -> Int8,
        parent -> Nullable<Varchar>,
    }
}

diesel::table! {
    vms (key) {
        key -> Varchar,
        created_at -> Timestamptz,
        name -> Varchar,
        config_key -> Uuid,
        namespace_name -> Varchar,
    }
}

diesel::joinable!(cargoes -> cargo_configs (config_key));
diesel::joinable!(cargoes -> namespaces (namespace_name));
diesel::joinable!(node_group_links -> node_groups (node_group_name));
diesel::joinable!(node_group_links -> nodes (node_name));
diesel::joinable!(resource_kind_versions -> resource_kinds (resource_kind_name));
diesel::joinable!(resources -> resource_configs (config_key));
diesel::joinable!(vms -> namespaces (namespace_name));
diesel::joinable!(vms -> vm_configs (config_key));

diesel::allow_tables_to_appear_in_same_query!(
    cargo_configs,
    cargoes,
    http_metrics,
    metrics,
    namespaces,
    node_group_links,
    node_groups,
    nodes,
    resource_configs,
    resource_kind_versions,
    resource_kinds,
    resources,
    vm_configs,
    vm_images,
    vms,
);
