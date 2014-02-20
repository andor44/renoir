#[allow(unused_imports)];

pub mod PackData {
  use std;
  use capnp::any::AnyPointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::blob::{Text, Data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader};
  use capnp::list::{PrimitiveList, ToU16, EnumList, StructList, TextList, DataList, ListList};
  use comm::pack_capnp;

  pub static STRUCT_SIZE : layout::StructSize =
    layout::StructSize { data : 1, pointers : 1, preferred_list_encoding : layout::INLINE_COMPOSITE};


  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> Reader<'a> {
    pub fn has_pubkey(&self) -> bool {
      if self.reader.get_data_field::<u16>(0) != 0 { return false; }
      !self.reader.get_pointer_field(0).is_null()
    }
    pub fn has_message(&self) -> bool {
      if self.reader.get_data_field::<u16>(0) != 2 { return false; }
      !self.reader.get_pointer_field(0).is_null()
    }
    pub fn has_quit(&self) -> bool {
      if self.reader.get_data_field::<u16>(0) != 4 { return false; }
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn which(&self) -> Option<WhichReader<'a>> {
      match self.reader.get_data_field::<u16>(0) {
        0 => {
          return std::option::Some(Pubkey(
            self.reader.get_pointer_field(0).get_data(std::ptr::null(), 0)
          ));
        }
        1 => {
          return std::option::Some(Login(
            FromStructReader::new(self.reader)
          ));
        }
        2 => {
          return std::option::Some(Message(
            FromStructReader::new(self.reader.get_pointer_field(0).get_struct( std::ptr::null()))
          ));
        }
        3 => {
          return std::option::Some(ResponseCode(
            self.reader.get_data_field::<u8>(2)
          ));
        }
        4 => {
          return std::option::Some(Quit(
            self.reader.get_pointer_field(0).get_text(std::ptr::null(), 0)
          ));
        }
        _ => return std::option::None
      }
    }
  }

  pub struct Builder<'a> { priv builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn set_pubkey(&self, value : Data::Reader) {
      self.builder.set_data_field::<u16>(0, 0);
      self.builder.get_pointer_field(0).set_data(value);
    }
    #[inline]
    pub fn init_pubkey(&self, size : uint) -> Data::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 0);
      self.builder.get_pointer_field(0).init_data(size)
    }
    pub fn has_pubkey(&self) -> bool {
      if self.builder.get_data_field::<u16>(0) != 0 { return false; }
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn init_login(&self, ) -> pack_capnp::PackData::Login::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 1);
      self.builder.get_pointer_field(0).clear();
      FromStructBuilder::new(self.builder)
    }
    #[inline]
    pub fn set_message(&self, value : pack_capnp::PackData::Message::Reader) {
      self.builder.set_data_field::<u16>(0, 2);
      self.builder.get_pointer_field(0).set_struct(&value.reader)
    }
    #[inline]
    pub fn init_message(&self, ) -> pack_capnp::PackData::Message::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 2);
      FromStructBuilder::new(self.builder.get_pointer_field(0).init_struct(pack_capnp::PackData::Message::STRUCT_SIZE))
    }
    pub fn has_message(&self) -> bool {
      if self.builder.get_data_field::<u16>(0) != 2 { return false; }
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn set_response_code(&self, value : u8) {
      self.builder.set_data_field::<u16>(0, 3);
      self.builder.set_data_field::<u8>(2, value);
    }
    #[inline]
    pub fn set_quit(&self, value : Text::Reader) {
      self.builder.set_data_field::<u16>(0, 4);
      self.builder.get_pointer_field(0).set_text(value);
    }
    #[inline]
    pub fn init_quit(&self, size : uint) -> Text::Builder<'a> {
      self.builder.set_data_field::<u16>(0, 4);
      self.builder.get_pointer_field(0).init_text(size)
    }
    pub fn has_quit(&self) -> bool {
      if self.builder.get_data_field::<u16>(0) != 4 { return false; }
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn which(&self) -> Option<Which::WhichBuilder<'a>> {
      match self.builder.get_data_field::<u16>(0) {
        0 => {
          return std::option::Some(Which::Pubkey(
            self.builder.get_pointer_field(0).get_data(std::ptr::null(), 0)
          ));
        }
        1 => {
          return std::option::Some(Which::Login(
            FromStructBuilder::new(self.builder)
          ));
        }
        2 => {
          return std::option::Some(Which::Message(
            FromStructBuilder::new(self.builder.get_pointer_field(0).get_struct(pack_capnp::PackData::Message::STRUCT_SIZE, std::ptr::null()))
          ));
        }
        3 => {
          return std::option::Some(Which::ResponseCode(
            self.builder.get_data_field::<u8>(2)
          ));
        }
        4 => {
          return std::option::Some(Which::Quit(
            self.builder.get_pointer_field(0).get_text(std::ptr::null(), 0)
          ));
        }
        _ => return std::option::None
      }
    }
  }

  pub struct Pipeline { priv _typeless : AnyPointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
  }
  pub enum WhichReader<'a> {
    Pubkey(Data::Reader<'a>),
    Login(pack_capnp::PackData::Login::Reader<'a>),
    Message(pack_capnp::PackData::Message::Reader<'a>),
    ResponseCode(u8),
    Quit(Text::Reader<'a>),
  }
  pub mod Which {
    use std;
    use capnp::any::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::blob::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader};
    use capnp::list::{PrimitiveList, ToU16, EnumList, StructList, TextList, DataList, ListList};
    use comm::pack_capnp;

    pub enum WhichBuilder<'a> {
      Pubkey(Data::Builder<'a>),
      Login(pack_capnp::PackData::Login::Builder<'a>),
      Message(pack_capnp::PackData::Message::Builder<'a>),
      ResponseCode(u8),
      Quit(Text::Builder<'a>),
    }
  }

  pub mod Message {
    use std;
    use capnp::any::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::blob::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader};
    use capnp::list::{PrimitiveList, ToU16, EnumList, StructList, TextList, DataList, ListList};
    use comm::pack_capnp;

    pub static STRUCT_SIZE : layout::StructSize =
      layout::StructSize { data : 1, pointers : 2, preferred_list_encoding : layout::INLINE_COMPOSITE};


    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_target(&self) -> pack_capnp::PackData::Message::Target::Reader<'a> {
        FromStructReader::new(self.reader)
      }
      #[inline]
      pub fn get_message(&self) -> Text::Reader<'a> {
        self.reader.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      pub fn has_message(&self) -> bool {
        !self.reader.get_pointer_field(1).is_null()
      }
    }

    pub struct Builder<'a> { priv builder : layout::StructBuilder<'a> }
    impl <'a> layout::HasStructSize for Builder<'a> {
      #[inline]
      fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
    }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_target(&self) -> pack_capnp::PackData::Message::Target::Builder<'a> {
        FromStructBuilder::new(self.builder)
      }
      #[inline]
      pub fn init_target(&self, ) -> pack_capnp::PackData::Message::Target::Builder<'a> {
        self.builder.set_data_field::<u16>(0, 0);
        self.builder.get_pointer_field(0).clear();
        FromStructBuilder::new(self.builder)
      }
      #[inline]
      pub fn get_message(&self) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).get_text(std::ptr::null(), 0)
      }
      #[inline]
      pub fn set_message(&self, value : Text::Reader) {
        self.builder.get_pointer_field(1).set_text(value);
      }
      #[inline]
      pub fn init_message(&self, size : uint) -> Text::Builder<'a> {
        self.builder.get_pointer_field(1).init_text(size)
      }
      pub fn has_message(&self) -> bool {
        !self.builder.get_pointer_field(1).is_null()
      }
    }

    pub struct Pipeline { priv _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
      pub fn get_target(&self) -> pack_capnp::PackData::Message::Target::Pipeline {
        FromTypelessPipeline::new(self._typeless.noop())
      }
    }

    pub mod Target {
      use std;
      use capnp::any::AnyPointer;
      use capnp::capability::{FromClientHook, FromTypelessPipeline};
      use capnp::blob::{Text, Data};
      use capnp::layout;
      use capnp::layout::{FromStructBuilder, FromStructReader};
      use capnp::list::{PrimitiveList, ToU16, EnumList, StructList, TextList, DataList, ListList};
      use comm::pack_capnp;

      pub struct Reader<'a> { reader : layout::StructReader<'a> }

      impl <'a> layout::FromStructReader<'a> for Reader<'a> {
        fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
          Reader { reader : reader }
        }
      }

      impl <'a> Reader<'a> {
        pub fn has_private(&self) -> bool {
          if self.reader.get_data_field::<u16>(0) != 2 { return false; }
          !self.reader.get_pointer_field(0).is_null()
        }
        #[inline]
        pub fn which(&self) -> Option<WhichReader<'a>> {
          match self.reader.get_data_field::<u16>(0) {
            0 => {
              return std::option::Some(Public(
                ()
              ));
            }
            1 => {
              return std::option::Some(Announcement(
                ()
              ));
            }
            2 => {
              return std::option::Some(Private(
                self.reader.get_pointer_field(0).get_text(std::ptr::null(), 0)
              ));
            }
            _ => return std::option::None
          }
        }
      }

      pub struct Builder<'a> { priv builder : layout::StructBuilder<'a> }
      impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
        fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
          Builder { builder : builder }
        }
      }
      impl <'a> Builder<'a> {
        pub fn as_reader(&self) -> Reader<'a> {
          FromStructReader::new(self.builder.as_reader())
        }
        #[inline]
        pub fn set_public(&self, _value : ()) {
          self.builder.set_data_field::<u16>(0, 0);
        }
        #[inline]
        pub fn set_announcement(&self, _value : ()) {
          self.builder.set_data_field::<u16>(0, 1);
        }
        #[inline]
        pub fn set_private(&self, value : Text::Reader) {
          self.builder.set_data_field::<u16>(0, 2);
          self.builder.get_pointer_field(0).set_text(value);
        }
        #[inline]
        pub fn init_private(&self, size : uint) -> Text::Builder<'a> {
          self.builder.set_data_field::<u16>(0, 2);
          self.builder.get_pointer_field(0).init_text(size)
        }
        pub fn has_private(&self) -> bool {
          if self.builder.get_data_field::<u16>(0) != 2 { return false; }
          !self.builder.get_pointer_field(0).is_null()
        }
        #[inline]
        pub fn which(&self) -> Option<Which::WhichBuilder<'a>> {
          match self.builder.get_data_field::<u16>(0) {
            0 => {
              return std::option::Some(Which::Public(
                ()
              ));
            }
            1 => {
              return std::option::Some(Which::Announcement(
                ()
              ));
            }
            2 => {
              return std::option::Some(Which::Private(
                self.builder.get_pointer_field(0).get_text(std::ptr::null(), 0)
              ));
            }
            _ => return std::option::None
          }
        }
      }

      pub struct Pipeline { priv _typeless : AnyPointer::Pipeline }
      impl FromTypelessPipeline for Pipeline {
        fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
          Pipeline { _typeless : typeless }
        }
      }
      impl Pipeline {
      }
      pub enum WhichReader<'a> {
        Public(()),
        Announcement(()),
        Private(Text::Reader<'a>),
      }
      pub mod Which {
        use std;
        use capnp::any::AnyPointer;
        use capnp::capability::{FromClientHook, FromTypelessPipeline};
        use capnp::blob::{Text, Data};
        use capnp::layout;
        use capnp::layout::{FromStructBuilder, FromStructReader};
        use capnp::list::{PrimitiveList, ToU16, EnumList, StructList, TextList, DataList, ListList};
        use comm::pack_capnp;

        pub enum WhichBuilder<'a> {
          Public(()),
          Announcement(()),
          Private(Text::Builder<'a>),
        }
      }
    }
  }

  pub mod Login {
    use std;
    use capnp::any::AnyPointer;
    use capnp::capability::{FromClientHook, FromTypelessPipeline};
    use capnp::blob::{Text, Data};
    use capnp::layout;
    use capnp::layout::{FromStructBuilder, FromStructReader};
    use capnp::list::{PrimitiveList, ToU16, EnumList, StructList, TextList, DataList, ListList};
    use comm::pack_capnp;

    pub struct Reader<'a> { reader : layout::StructReader<'a> }

    impl <'a> layout::FromStructReader<'a> for Reader<'a> {
      fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
        Reader { reader : reader }
      }
    }

    impl <'a> Reader<'a> {
      #[inline]
      pub fn get_name(&self) -> Text::Reader<'a> {
        self.reader.get_pointer_field(0).get_text(std::ptr::null(), 0)
      }
      pub fn has_name(&self) -> bool {
        !self.reader.get_pointer_field(0).is_null()
      }
    }

    pub struct Builder<'a> { priv builder : layout::StructBuilder<'a> }
    impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
      fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
        Builder { builder : builder }
      }
    }
    impl <'a> Builder<'a> {
      pub fn as_reader(&self) -> Reader<'a> {
        FromStructReader::new(self.builder.as_reader())
      }
      #[inline]
      pub fn get_name(&self) -> Text::Builder<'a> {
        self.builder.get_pointer_field(0).get_text(std::ptr::null(), 0)
      }
      #[inline]
      pub fn set_name(&self, value : Text::Reader) {
        self.builder.get_pointer_field(0).set_text(value);
      }
      #[inline]
      pub fn init_name(&self, size : uint) -> Text::Builder<'a> {
        self.builder.get_pointer_field(0).init_text(size)
      }
      pub fn has_name(&self) -> bool {
        !self.builder.get_pointer_field(0).is_null()
      }
    }

    pub struct Pipeline { priv _typeless : AnyPointer::Pipeline }
    impl FromTypelessPipeline for Pipeline {
      fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
        Pipeline { _typeless : typeless }
      }
    }
    impl Pipeline {
    }
  }
}

pub mod Pack {
  use std;
  use capnp::any::AnyPointer;
  use capnp::capability::{FromClientHook, FromTypelessPipeline};
  use capnp::blob::{Text, Data};
  use capnp::layout;
  use capnp::layout::{FromStructBuilder, FromStructReader};
  use capnp::list::{PrimitiveList, ToU16, EnumList, StructList, TextList, DataList, ListList};
  use comm::pack_capnp;

  pub static STRUCT_SIZE : layout::StructSize =
    layout::StructSize { data : 0, pointers : 2, preferred_list_encoding : layout::INLINE_COMPOSITE};


  pub struct Reader<'a> { reader : layout::StructReader<'a> }

  impl <'a> layout::FromStructReader<'a> for Reader<'a> {
    fn new(reader: layout::StructReader<'a>) -> Reader<'a> {
      Reader { reader : reader }
    }
  }

  impl <'a> Reader<'a> {
    #[inline]
    pub fn get_nonce(&self) -> Data::Reader<'a> {
      self.reader.get_pointer_field(0).get_data(std::ptr::null(), 0)
    }
    pub fn has_nonce(&self) -> bool {
      !self.reader.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_data(&self) -> Data::Reader<'a> {
      self.reader.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    pub fn has_data(&self) -> bool {
      !self.reader.get_pointer_field(1).is_null()
    }
  }

  pub struct Builder<'a> { priv builder : layout::StructBuilder<'a> }
  impl <'a> layout::HasStructSize for Builder<'a> {
    #[inline]
    fn struct_size(_unused_self : Option<Builder>) -> layout::StructSize { STRUCT_SIZE }
  }
  impl <'a> layout::FromStructBuilder<'a> for Builder<'a> {
    fn new(builder : layout::StructBuilder<'a>) -> Builder<'a> {
      Builder { builder : builder }
    }
  }
  impl <'a> Builder<'a> {
    pub fn as_reader(&self) -> Reader<'a> {
      FromStructReader::new(self.builder.as_reader())
    }
    #[inline]
    pub fn get_nonce(&self) -> Data::Builder<'a> {
      self.builder.get_pointer_field(0).get_data(std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_nonce(&self, value : Data::Reader) {
      self.builder.get_pointer_field(0).set_data(value);
    }
    #[inline]
    pub fn init_nonce(&self, size : uint) -> Data::Builder<'a> {
      self.builder.get_pointer_field(0).init_data(size)
    }
    pub fn has_nonce(&self) -> bool {
      !self.builder.get_pointer_field(0).is_null()
    }
    #[inline]
    pub fn get_data(&self) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).get_data(std::ptr::null(), 0)
    }
    #[inline]
    pub fn set_data(&self, value : Data::Reader) {
      self.builder.get_pointer_field(1).set_data(value);
    }
    #[inline]
    pub fn init_data(&self, size : uint) -> Data::Builder<'a> {
      self.builder.get_pointer_field(1).init_data(size)
    }
    pub fn has_data(&self) -> bool {
      !self.builder.get_pointer_field(1).is_null()
    }
  }

  pub struct Pipeline { priv _typeless : AnyPointer::Pipeline }
  impl FromTypelessPipeline for Pipeline {
    fn new(typeless : AnyPointer::Pipeline) -> Pipeline {
      Pipeline { _typeless : typeless }
    }
  }
  impl Pipeline {
  }
}
