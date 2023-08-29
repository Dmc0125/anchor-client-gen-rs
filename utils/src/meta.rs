use std::collections::HashSet;

use crate::idl::{
    EnumVariantJsonDefinition, EventJsonDefinition, FieldJsonDefinition, FieldTypeJsonDefinition,
    IdlJsonDefinition, TypeDefJsonDefinition, TypedefTypeJsonDefinition,
};

pub trait Attributes {
    fn can_copy(&self, meta: &mut Meta) -> bool;

    fn can_default(&self, meta: &mut Meta) -> bool;

    fn has_defined(&self) -> bool;
}

impl Attributes for FieldTypeJsonDefinition {
    fn can_copy(&self, meta: &mut Meta) -> bool {
        match self {
            Self::Array { .. } => false,
            Self::Defined { defined } => meta.can_copy.get(defined).map_or(false, |_| true),
            Self::Option { option } => option.can_copy(meta),
            Self::Primitive(primitive) => match primitive.as_str() {
                "String" => false,
                _ => true,
            },
            Self::Vec { .. } => false,
        }
    }

    fn can_default(&self, meta: &mut Meta) -> bool {
        match self {
            Self::Array { .. } => false,
            Self::Defined { defined } => meta.can_default.get(defined).map_or(false, |_| true),
            Self::Option { option } => option.can_default(meta),
            _ => true,
        }
    }

    fn has_defined(&self) -> bool {
        match self {
            Self::Defined { .. } => true,
            Self::Array { array } => array.0.has_defined(),
            Self::Vec { vec } => vec.has_defined(),
            Self::Option { option } => option.has_defined(),
            _ => false,
        }
    }
}

impl Attributes for FieldJsonDefinition {
    fn can_copy(&self, meta: &mut Meta) -> bool {
        self.field_type.can_copy(meta)
    }

    fn can_default(&self, meta: &mut Meta) -> bool {
        self.field_type.can_default(meta)
    }

    fn has_defined(&self) -> bool {
        self.field_type.has_defined()
    }
}

impl Attributes for EnumVariantJsonDefinition {
    fn can_copy(&self, meta: &mut Meta) -> bool {
        match self {
            Self::Struct { fields, .. } => fields.iter().all(|field| field.can_copy(meta)),
            Self::Tuple { types, .. } => types.iter().all(|el_type| el_type.can_copy(meta)),
            Self::UnitLike { .. } => true,
        }
    }

    fn can_default(&self, _: &mut Meta) -> bool {
        false
    }

    fn has_defined(&self) -> bool {
        match self {
            Self::Struct { fields, .. } => fields.iter().any(|field| field.has_defined()),
            Self::Tuple { types, .. } => types.iter().any(|el_type| el_type.has_defined()),
            Self::UnitLike { .. } => false,
        }
    }
}

impl Attributes for TypedefTypeJsonDefinition {
    fn can_copy(&self, meta: &mut Meta) -> bool {
        match self {
            Self::Enum { variants, .. } => variants.iter().all(|variant| variant.can_copy(meta)),
            Self::Struct { fields, .. } => fields.iter().all(|field| field.can_copy(meta)),
        }
    }

    fn can_default(&self, meta: &mut Meta) -> bool {
        match self {
            Self::Enum { variants, .. } => variants.iter().all(|variant| variant.can_default(meta)),
            Self::Struct { fields, .. } => fields.iter().all(|field| field.can_default(meta)),
        }
    }

    fn has_defined(&self) -> bool {
        match self {
            Self::Enum { variants, .. } => variants.iter().any(|variant| variant.has_defined()),
            Self::Struct { fields, .. } => fields.iter().any(|field| field.has_defined()),
        }
    }
}

impl Attributes for TypeDefJsonDefinition {
    fn can_copy(&self, meta: &mut Meta) -> bool {
        if self.typedef_type.can_copy(meta) {
            meta.can_copy.insert(self.name.clone());
            true
        } else {
            false
        }
    }

    fn can_default(&self, meta: &mut Meta) -> bool {
        if self.typedef_type.can_default(meta) {
            meta.can_default.insert(self.name.clone());
            true
        } else {
            false
        }
    }

    fn has_defined(&self) -> bool {
        self.typedef_type.has_defined()
    }
}

impl Attributes for EventJsonDefinition {
    fn can_copy(&self, meta: &mut Meta) -> bool {
        let can_copy = self.fields.iter().all(|field| field.can_copy(meta));
        if can_copy {
            meta.can_copy.insert(self.name.clone());
            true
        } else {
            false
        }
    }

    fn can_default(&self, meta: &mut Meta) -> bool {
        let can_default = self.fields.iter().all(|field| field.can_default(meta));
        if can_default {
            meta.can_default.insert(self.name.clone());
            true
        } else {
            false
        }
    }

    fn has_defined(&self) -> bool {
        self.fields.iter().any(|field| field.has_defined())
    }
}

#[derive(Debug)]
pub struct Meta {
    pub can_copy: HashSet<String>,
    pub can_default: HashSet<String>,
}

impl From<&IdlJsonDefinition> for Meta {
    fn from(idl: &IdlJsonDefinition) -> Self {
        let mut meta = Self {
            can_copy: Default::default(),
            can_default: Default::default(),
        };

        let (defined_types, non_defined_types): (Vec<_>, Vec<_>) =
            idl.types.iter().partition(|typedef| typedef.has_defined());

        let (defined_accounts, non_defined_accounts): (Vec<_>, Vec<_>) = idl
            .accounts
            .iter()
            .partition(|typedef| typedef.has_defined());

        let (defined_events, non_defined_events): (Vec<_>, Vec<_>) =
            idl.events.iter().partition(|typedef| typedef.has_defined());

        non_defined_types.iter().for_each(|typedef| {
            typedef.can_copy(&mut meta);
            typedef.can_default(&mut meta);
        });
        non_defined_accounts.iter().for_each(|typedef| {
            typedef.can_copy(&mut meta);
            typedef.can_default(&mut meta);
        });
        non_defined_events.iter().for_each(|typedef| {
            typedef.can_copy(&mut meta);
            typedef.can_default(&mut meta);
        });

        defined_types.iter().for_each(|typedef| {
            typedef.can_copy(&mut meta);
            typedef.can_default(&mut meta);
        });
        defined_accounts.iter().for_each(|typedef| {
            typedef.can_copy(&mut meta);
            typedef.can_default(&mut meta);
        });
        defined_events.iter().for_each(|typedef| {
            typedef.can_copy(&mut meta);
            typedef.can_default(&mut meta);
        });

        meta
    }
}
