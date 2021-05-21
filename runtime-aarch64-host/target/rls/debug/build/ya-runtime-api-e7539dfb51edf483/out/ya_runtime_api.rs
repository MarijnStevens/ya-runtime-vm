#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Request {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(oneof="request::Command", tags="2, 10, 11, 12")]
    pub command: ::std::option::Option<request::Command>,
}
pub mod request {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Hello {
        #[prost(string, tag="1")]
        pub version: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RunProcess {
        #[prost(string, tag="1")]
        pub bin: std::string::String,
        #[prost(string, repeated, tag="2")]
        pub args: ::std::vec::Vec<std::string::String>,
        #[prost(string, tag="3")]
        pub work_dir: std::string::String,
        #[prost(message, optional, tag="4")]
        pub stdout: ::std::option::Option<super::Output>,
        #[prost(message, optional, tag="5")]
        pub stderr: ::std::option::Option<super::Output>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KillProcess {
        #[prost(uint64, tag="1")]
        pub pid: u64,
        #[prost(int32, tag="2")]
        pub signal: i32,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Shutdown {
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag="2")]
        Hello(Hello),
        #[prost(message, tag="10")]
        Run(RunProcess),
        #[prost(message, tag="11")]
        Kill(KillProcess),
        #[prost(message, tag="12")]
        Shutdown(Shutdown),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Response {
    /// if false then id represents correlation id for response message.
    #[prost(bool, tag="1")]
    pub event: bool,
    #[prost(uint64, tag="2")]
    pub id: u64,
    #[prost(oneof="response::Command", tags="3, 4, 10, 11, 12, 20")]
    pub command: ::std::option::Option<response::Command>,
}
pub mod response {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Hello {
        #[prost(string, tag="1")]
        pub version: std::string::String,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Error {
        #[prost(enumeration="ErrorCode", tag="1")]
        pub code: i32,
        #[prost(string, tag="2")]
        pub message: std::string::String,
        #[prost(map="string, string", tag="3")]
        pub context: ::std::collections::HashMap<std::string::String, std::string::String>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct RunProcess {
        #[prost(uint64, tag="1")]
        pub pid: u64,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct KillProcess {
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ProcessStatus {
        #[prost(uint64, tag="1")]
        pub pid: u64,
        #[prost(bool, tag="2")]
        pub running: bool,
        #[prost(int32, tag="3")]
        pub return_code: i32,
        #[prost(bytes, tag="4")]
        pub stdout: std::vec::Vec<u8>,
        #[prost(bytes, tag="5")]
        pub stderr: std::vec::Vec<u8>,
    }
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct Shutdown {
    }
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
    #[repr(i32)]
    pub enum ErrorCode {
        Internal = 0,
        NotFound = 1,
        BadRequest = 2,
    }
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Command {
        #[prost(message, tag="3")]
        Error(Error),
        #[prost(message, tag="4")]
        Hello(Hello),
        #[prost(message, tag="10")]
        Run(RunProcess),
        #[prost(message, tag="11")]
        Kill(KillProcess),
        #[prost(message, tag="12")]
        Shutdown(Shutdown),
        /// Events
        #[prost(message, tag="20")]
        Status(ProcessStatus),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Output {
    /// No-type = /dev/null
    /// at_end(buffer size) = last n bytes are returned.
    #[prost(oneof="output::Type", tags="2")]
    pub r#type: ::std::option::Option<output::Type>,
}
pub mod output {
    /// No-type = /dev/null
    /// at_end(buffer size) = last n bytes are returned.
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Type {
        #[prost(uint32, tag="2")]
        AtEnd(u32),
    }
}
