use crate::reader::decoding::*;
use crate::{
    header::AccessFlags,
    reader::{cpool, Attribute},
};
use crate::{mutf8, MStr};

use super::FromAttribute;

dec_structure! {
    pub struct EnclosingMethod<'input> into {
        class: cpool::Index<cpool::Class<'input>>,
        method: Option<cpool::Index<cpool::NameAndType<'input>>>,
    }
}

impl<'input> FromAttribute<'input> for EnclosingMethod<'input> {
    const NAME: &'static MStr = mutf8!("EnclosingMethod");
}

dec_structure! {
    pub struct NestHost<'input> into {
        host_class: cpool::Index<cpool::Class<'input>>,
    }
}

impl<'input> FromAttribute<'input> for NestHost<'input> {
    const NAME: &'static MStr = mutf8!("NestHost");
}

dec_structure! {
    pub struct NestMembers<'input> into {
        classes: DecodeMany<'input, cpool::Index<cpool::Class<'input>>, u16>,
    }
}

impl<'input> FromAttribute<'input> for NestMembers<'input> {
    const NAME: &'static MStr = mutf8!("NestMembers");
}

dec_structure! {
    pub struct InnerClasses<'input> into {
        classes: DecodeMany<'input, InnerClass<'input>, u16>,
    }
}

impl<'input> FromAttribute<'input> for InnerClasses<'input> {
    const NAME: &'static MStr = mutf8!("InnerClasses");
}

dec_structure! {
    pub struct InnerClass<'input> {
        inner_class: cpool::Index<cpool::Class<'input>>,
        outer_class: Option<cpool::Index<cpool::Class<'input>>>,
        inner_name: Option<cpool::Index<cpool::Utf8<'input>>>,
        inner_access_flags: AccessFlags,
    }
}

dec_structure! {
    pub struct BootstrapMethods<'input> into {
        methods: DecodeMany<'input, BootstrapMethod<'input>, u16>,
    }
}

impl<'input> FromAttribute<'input> for BootstrapMethods<'input> {
    const NAME: &'static MStr = mutf8!("BootstrapMethods");
}

dec_structure! {
    pub struct BootstrapMethod<'input> {
        method_ref: cpool::Index<cpool::MethodHandle<'input>>,
        arguments: DecodeMany<'input, cpool::Index<cpool::Item<'input>>, u16>
    }
}

dec_structure! {
    pub struct Record<'input> into {
        components: DecodeMany<'input, RecordComponent<'input>, u16>,
    }
}

dec_structure! {
    pub struct RecordComponent<'input> {
        name: cpool::Index<cpool::Utf8<'input>>,
        descriptor: cpool::Index<cpool::Utf8<'input>>,
        attributes: DecodeMany<'input, Attribute<'input>, u16>,
    }
}

dec_structure! {
    pub struct PermittedSubclasses<'input> into {
        classes: DecodeMany<'input, cpool::Index<cpool::Class<'input>>, u16>,
    }
}
