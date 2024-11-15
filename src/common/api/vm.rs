use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    CustomResourceDefinitionSpec, CustomResourceDefinitionStatus,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use k8s_openapi::serde::de::{Error, IgnoredAny, MapAccess, Unexpected};
use k8s_openapi::serde::ser::SerializeStruct;
use k8s_openapi::{ClusterResourceScope, Resource};

use crate::config::crd;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Vm {
    pub metadata: ObjectMeta,
    pub spec: CustomResourceDefinitionSpec,
    pub status: Option<CustomResourceDefinitionStatus>,
}

impl Resource for Vm {
    const API_VERSION: &'static str = crd::VM_API_VERSION;
    const GROUP: &'static str = crd::VM_GROUP;
    const KIND: &'static str = crd::VM;
    const VERSION: &'static str = crd::VM_VERSION;
    const URL_PATH_SEGMENT: &'static str = crd::VM_URL_PATH_SEGMENT;
    type Scope = ClusterResourceScope;
}
impl k8s_openapi::ListableResource for Vm {
    const LIST_KIND: &'static str = "VmList";
}

impl k8s_openapi::Metadata for Vm {
    type Ty = ObjectMeta;

    fn metadata(&self) -> &<Self as k8s_openapi::Metadata>::Ty {
        &self.metadata
    }

    fn metadata_mut(&mut self) -> &mut <Self as k8s_openapi::Metadata>::Ty {
        &mut self.metadata
    }
}

impl k8s_openapi::DeepMerge for Vm {
    fn merge_from(&mut self, other: Self) {
        k8s_openapi::DeepMerge::merge_from(&mut self.metadata, other.metadata);
        k8s_openapi::DeepMerge::merge_from(&mut self.spec, other.spec);
        k8s_openapi::DeepMerge::merge_from(&mut self.status, other.status);
    }
}

impl<'de> k8s_openapi::serde::Deserialize<'de> for Vm {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: k8s_openapi::serde::Deserializer<'de>,
    {
        #[allow(non_camel_case_types)]
        enum Field {
            Key_api_version,
            Key_kind,
            Key_metadata,
            Key_spec,
            Key_status,
            Other,
        }

        impl<'de> k8s_openapi::serde::Deserialize<'de> for Field {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: k8s_openapi::serde::Deserializer<'de>,
            {
                struct Visitor;

                impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
                    type Value = Field;

                    fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.write_str("field identifier")
                    }

                    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
                    where
                        E: k8s_openapi::serde::de::Error,
                    {
                        Ok(match v {
                            "apiVersion" => Field::Key_api_version,
                            "kind" => Field::Key_kind,
                            "metadata" => Field::Key_metadata,
                            "spec" => Field::Key_spec,
                            "status" => Field::Key_status,
                            _ => Field::Other,
                        })
                    }
                }

                deserializer.deserialize_identifier(Visitor)
            }
        }

        struct Visitor;

        impl<'de> k8s_openapi::serde::de::Visitor<'de> for Visitor {
            type Value = Vm;

            fn expecting(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str(<Self::Value as Resource>::KIND)
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: MapAccess<'de>,
            {
                let mut value_metadata: Option<ObjectMeta> = None;
                let mut value_spec: Option<CustomResourceDefinitionSpec> = None;
                let mut value_status: Option<CustomResourceDefinitionStatus> = None;

                while let Some(key) = MapAccess::next_key::<Field>(&mut map)? {
                    match key {
                        Field::Key_api_version => {
                            let value_api_version: String = MapAccess::next_value(&mut map)?;
                            if value_api_version != <Self::Value as Resource>::API_VERSION {
                                return Err(Error::invalid_value(
                                    Unexpected::Str(&value_api_version),
                                    &<Self::Value as Resource>::API_VERSION,
                                ));
                            }
                        }
                        Field::Key_kind => {
                            let value_kind: String = MapAccess::next_value(&mut map)?;
                            if value_kind != <Self::Value as Resource>::KIND {
                                return Err(Error::invalid_value(
                                    Unexpected::Str(&value_kind),
                                    &<Self::Value as Resource>::KIND,
                                ));
                            }
                        }
                        Field::Key_metadata => value_metadata = MapAccess::next_value(&mut map)?,
                        Field::Key_spec => value_spec = MapAccess::next_value(&mut map)?,
                        Field::Key_status => value_status = MapAccess::next_value(&mut map)?,
                        Field::Other => {
                            let _: IgnoredAny = MapAccess::next_value(&mut map)?;
                        }
                    }
                }

                Ok(Vm {
                    metadata: value_metadata.unwrap_or_default(),
                    spec: value_spec.unwrap_or_default(),
                    status: value_status,
                })
            }
        }

        deserializer.deserialize_struct(
            <Self as Resource>::KIND,
            &["apiVersion", "kind", "metadata", "spec", "status"],
            Visitor,
        )
    }
}

impl k8s_openapi::serde::Serialize for Vm {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: k8s_openapi::serde::Serializer,
    {
        let mut state = serializer.serialize_struct(
            <Self as Resource>::KIND,
            4 + self.status.as_ref().map_or(0, |_| 1),
        )?;
        SerializeStruct::serialize_field(
            &mut state,
            "apiVersion",
            <Self as Resource>::API_VERSION,
        )?;
        SerializeStruct::serialize_field(&mut state, "kind", <Self as Resource>::KIND)?;
        SerializeStruct::serialize_field(&mut state, "metadata", &self.metadata)?;
        SerializeStruct::serialize_field(&mut state, "spec", &self.spec)?;
        if let Some(value) = &self.status {
            SerializeStruct::serialize_field(&mut state, "status", value)?;
        }
        SerializeStruct::end(state)
    }
}

#[cfg(feature = "schemars")]
impl k8s_openapi::schemars::JsonSchema for Vm {
    fn schema_name() -> String {
        "io.k8s.api.core.v1.Vm".to_owned()
    }

    fn json_schema(
        __gen: &mut k8s_openapi::schemars::gen::SchemaGenerator,
    ) -> k8s_openapi::schemars::schema::Schema {
        k8s_openapi::schemars::schema::Schema::Object(k8s_openapi::schemars::schema::SchemaObject {
            metadata: Some(Box::new(k8s_openapi::schemars::schema::Metadata {
                description: Some("Vm is a worker node in Kubernetes. Each host will have a unique identifier in the cache (i.e. in etcd).".to_owned()),
                ..Default::default()
            })),
            instance_type: Some(k8s_openapi::schemars::schema::SingleOrVec::Single(Box::new(k8s_openapi::schemars::schema::InstanceType::Object))),
            object: Some(Box::new(k8s_openapi::schemars::schema::ObjectValidation {
                properties: [
                    (
                        "apiVersion".to_owned(),
                        k8s_openapi::schemars::schema::Schema::Object(k8s_openapi::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(k8s_openapi::schemars::schema::Metadata {
                                description: Some("APIVersion defines the versioned schema of this representation of an object. Servers should convert recognized schemas to the latest internal value, and may reject unrecognized values. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#resources".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(k8s_openapi::schemars::schema::SingleOrVec::Single(Box::new(k8s_openapi::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "kind".to_owned(),
                        k8s_openapi::schemars::schema::Schema::Object(k8s_openapi::schemars::schema::SchemaObject {
                            metadata: Some(Box::new(k8s_openapi::schemars::schema::Metadata {
                                description: Some("Kind is a string value representing the REST resource this object represents. Servers may infer this from the endpoint the client submits requests to. Cannot be updated. In CamelCase. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#types-kinds".to_owned()),
                                ..Default::default()
                            })),
                            instance_type: Some(k8s_openapi::schemars::schema::SingleOrVec::Single(Box::new(k8s_openapi::schemars::schema::InstanceType::String))),
                            ..Default::default()
                        }),
                    ),
                    (
                        "metadata".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<ObjectMeta>().into_object();
                            schema_obj.metadata = Some(Box::new(k8s_openapi::schemars::schema::Metadata {
                                description: Some("Standard object's metadata. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#metadata".to_owned()),
                                ..Default::default()
                            }));
                            k8s_openapi::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "spec".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<CustomResourceDefinitionSpec>().into_object();
                            schema_obj.metadata = Some(Box::new(k8s_openapi::schemars::schema::Metadata {
                                description: Some("Spec defines the behavior of a vm. https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".to_owned()),
                                ..Default::default()
                            }));
                            k8s_openapi::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                    (
                        "status".to_owned(),
                        {
                            let mut schema_obj = __gen.subschema_for::<CustomResourceDefinitionStatus>().into_object();
                            schema_obj.metadata = Some(Box::new(k8s_openapi::schemars::schema::Metadata {
                                description: Some("Most recently observed status of the vm. Populated by the system. Read-only. More info: https://git.k8s.io/community/contributors/devel/sig-architecture/api-conventions.md#spec-and-status".to_owned()),
                                ..Default::default()
                            }));
                            k8s_openapi::schemars::schema::Schema::Object(schema_obj)
                        },
                    ),
                ].into(),
                required: [
                    "metadata".to_owned(),
                ].into(),
                ..Default::default()
            })),
            ..Default::default()
        })
    }
}
