use crate::paths::InstancePath;
use crate::error::{ErrorIterator, error, no_error, ValidationError};
use crate::compilation::JSONSchema;

use serde_json::Value;
use std::fmt;

pub(crate) trait Validate: Send + Sync + ToString {
    fn validate<'a>(
        &self,
        schema: &'a JSONSchema,
        instance: &'a Value,
        instance_path: &InstancePath,
    ) -> ErrorIterator<'a>;
    // The same as above, but does not construct ErrorIterator.
    // It is faster for cases when the result is not needed (like anyOf), since errors are
    // not constructed
    fn is_valid(&self, schema: &JSONSchema, instance: &Value) -> bool;
}

impl fmt::Debug for dyn Validate + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.to_string())
    }
}
impl <T: crate::CustomFormat + Send + Sync + ToString> Validate for T {
    fn validate<'a>(
        &self,
        schema: &'a JSONSchema,
        instance: &'a Value,
        instance_path: &InstancePath,
    ) -> ErrorIterator<'a> {
        if let Value::String(_s) = instance {
            if !<T as Validate>::is_valid(&self,schema,instance) {
                error(ValidationError::format(
                    instance_path.into(),
                    instance,
                    T::NAME
                ))
            } else {
                no_error()
            }
        } else {
            no_error()
        }
    }

    fn is_valid(&self, _schema: &JSONSchema, instance: &Value) -> bool {
        if let Value::String(s) = instance{
            <T as crate::CustomFormat>::is_valid(&self, s)
        } else {
            true
        }
    }
}