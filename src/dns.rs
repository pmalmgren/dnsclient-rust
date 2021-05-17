use std::io::Error;
use std::net::{ToSocketAddrs, UdpSocket};
use serde::{Serialize, Deserialize};

/// DnsRecordType indicates the type of record being requested,
/// or the type of record being returned in a response.
#[derive(Clone, Copy, Debug)]
pub enum DnsRecordType {
    A = 1,
    NS = 2,
    CNAME = 5,
    SOA = 6,
    PTR = 12,
    MX = 15,
    TXT = 16,
    AAAA = 28,
    SRV = 33,
    NAPTR = 35,
    OPT = 41,
    IXFR = 251,
    AXFR = 252,
    ANY = 255,
}

impl DnsRecordType {
    fn value(&self) -> u8 {
        return *self as u8;
    }
}

/// DnsQueryType indicates how the server returns the responses.
#[derive(Clone, Copy, Debug)]
pub enum DnsQueryType {
    /// In an Iterative query type, the client is responsible for
    /// doing additional requests if the first nameserver does not
    /// have the data.
    Iterative = 0,
    /// In a Recursive query type, the server is responsible for
    /// doing additional requests if it does not have the data.
    Recursive,
}

impl DnsQueryType {
    fn value(&self) -> u16 {
        return *self as u16;
    }
}

/// DnsQueryClass indicates the class of the query.
#[derive(Clone, Copy, Debug)]
pub enum DnsQueryClass {
    InternetClass = 1,
    NoClass = 254,
    AllClass = 255,
}

/// QueryZone contains data for the Query/Zone section.
#[derive(Debug)]
pub struct QueryZone {
    qz_name: Box<str>,
    qz_type: DnsRecordType,
    qz_class: DnsQueryClass,
}

/// ResourceRecord contains data for answers, authority, and addditional
/// information sections.
#[derive(Debug)]
pub struct ResourceRecord {
    rr_name: Box<str>,

}

/// DnsMessageSection contains the data for both requests and responses.
/// The length can be variable, but is restricted to at most 416 bytes.
#[derive(Debug)]
pub struct DnsMessageSection {
    /// Queries and zone sections have their own format
    queries: Box<Vec<QueryZone>>,
    /// Answers, authority, and addditional information sections share
    /// a common format.
    answers: Box<Vec<ResourceRecord>>,
    authority: Box<Vec<ResourceRecord>>,
    additional: Box<Vec<ResourceRecord>>,
}

impl DnsMessageSection {
    fn new() -> Self {
        let queries: Vec<QueryZone> = Vec::with_capacity(1);
        let answers: Vec<ResourceRecord> = Vec::new();
        let authority: Vec<ResourceRecord> = Vec::new();
        let additional: Vec<ResourceRecord> = Vec::new();
        DnsMessageSection{
            queries: Box::new(queries),
            answers: Box::new(answers),
            authority: Box::new(authority),
            additional: Box::new(additional),
        }
    }
}

/// DnsMessage is the DNS message format for both requests and responses.
/// See RFC-6195 for more information about the fields.
#[derive(Debug)]
pub struct DnsMessage {
    /// Transaction ID is used by the client to match requests to responses
    transaction_id: u16,
    /// Flags are split into 10 fields
    flags: u16,
    /// The number of queries, generally 1 for a DNS request, 0 for response
    query_count: u16,
    /// The number of answers, generally 1 for a DNS response, 0 for request
    answer_count: u16,
    /// The number of authority messages
    authority_count: u16,
    /// The number of additional messages, used to reduce number of queries
    additional_count: u16,
    /// The data
    records: DnsMessageSection,
}

impl DnsMessage {
    pub fn new(trans_id: u16) -> Self {
        DnsMessage {
            transaction_id: trans_id,
            flags: 0,
            query_count: 0,
            answer_count: 0,
            authority_count: 0,
            additional_count: 0,
            records: DnsMessageSection::new(),
        }
    }

    fn set_query(&mut self, hostname: String, query: DnsQueryType, record: DnsRecordType) {
        // Flip QR (query), 1st bit of flags, to 1
        self.flags |= 0x8000;
        // Flip RD (recursion desired), 8th bit of flags, to specified value
        self.flags |= 0x80 * query.value();
        self.query_count = 1;
    }
}

#[derive(Debug)]
pub struct DnsSocket {
    udp_sock: UdpSocket,
    trans_id: u16,
}

impl DnsSocket {
    pub fn new<T: ToSocketAddrs>(server: T) -> Self {
        let udp_sock = UdpSocket::bind("0.0.0.0:0").unwrap();
        udp_sock.connect(server).unwrap();
        DnsSocket {
            udp_sock,
            trans_id: 0,
        }
    }

    pub fn query(
        &mut self,
        hostname: String,
        query: DnsQueryType,
        record: DnsRecordType,
    ) -> Result<DnsMessage, Error> {
        self.trans_id += 1;
        let mut dns_message = DnsMessage::new(self.trans_id);
        dns_message.set_query(hostname, query, record);

        Ok(dns_message)
    }
}
