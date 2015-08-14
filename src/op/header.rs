use super::op_code::OpCode;
use super::response_code::ResponseCode;
use super::super::rr::util;

/// RFC 1035        Domain Implementation and Specification    November 1987
///
/// 4.1.1. Header section format
///
/// The header contains the following fields:
///
///                                     1  1  1  1  1  1
///       0  1  2  3  4  5  6  7  8  9  0  1  2  3  4  5
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                      ID                       |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |QR|   Opcode  |AA|TC|RD|RA|   Z    |   RCODE   |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                    QDCOUNT                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                    ANCOUNT                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                    NSCOUNT                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///     |                    ARCOUNT                    |
///     +--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+--+
///
/// where:
///
/// ID              A 16 bit identifier assigned by the program that
///                 generates any kind of query.  This identifier is copied
///                 the corresponding reply and can be used by the requester
///                 to match up replies to outstanding queries.
///
/// QR              A one bit field that specifies whether this message is a
///                 query (0), or a response (1).
///
/// OPCODE          A four bit field that specifies kind of query in this
///                 message.  This value is set by the originator of a query
///                 and copied into the response.  The values are: <see super::op_code>
///
/// AA              Authoritative Answer - this bit is valid in responses,
///                 and specifies that the responding name server is an
///                 authority for the domain name in question section.
///
///                 Note that the contents of the answer section may have
///                 multiple owner names because of aliases.  The AA bit
///                 corresponds to the name which matches the query name, or
///                 the first owner name in the answer section.
///
/// TC              TrunCation - specifies that this message was truncated
///                 due to length greater than that permitted on the
///                 transmission channel.
///
/// RD              Recursion Desired - this bit may be set in a query and
///                 is copied into the response.  If RD is set, it directs
///                 the name server to pursue the query recursively.
///                 Recursive query support is optional.
///
/// RA              Recursion Available - this be is set or cleared in a
///                 response, and denotes whether recursive query support is
///                 available in the name server.
///
/// Z               Reserved for future use.  Must be zero in all queries
///                 and responses.
///
/// RCODE           Response code - this 4 bit field is set as part of
///                 responses.  The values have the following
///                 interpretation: <see super::response_code>
///
/// QDCOUNT         an unsigned 16 bit integer specifying the number of
///                 entries in the question section.
///
/// ANCOUNT         an unsigned 16 bit integer specifying the number of
///                 resource records in the answer section.
///
/// NSCOUNT         an unsigned 16 bit integer specifying the number of name
///                 server resource records in the authority records
///                 section.
///
/// ARCOUNT         an unsigned 16 bit integer specifying the number of
///                 resource records in the additional records section.
pub struct Header {
  id: u16, message_type: MessageType, op_code: OpCode,
  authoritative: bool, truncation: bool, recursion_desired: bool, recursion_available: bool,
  response_code: ResponseCode,
  question_count: u16, answer_count: u16, name_server_count: u16, additional_count: u16
}

enum MessageType {
  Query, Response
}

impl Header {
  pub fn parse(data: &mut Vec<u8>) -> Self {
    let id = util::parse_u16(data);

    let q_opcd_a_t_r = data.pop().unwrap_or(0);
    // if the first bit is set
    let message_type = if ((0x80 & q_opcd_a_t_r) == 0x80) { MessageType::Response } else { MessageType::Query };
    // the 4bit opcode, masked and then shifted right 3bits for the u8...
    let op_code: OpCode = ((0x78 & q_opcd_a_t_r) >> 3).into();
    let authoritative = (0x4 & q_opcd_a_t_r) == 0x4;
    let truncation = (0x2 & q_opcd_a_t_r) == 0x2;
    let recursion_desired = (0x1 & q_opcd_a_t_r) == 0x1;

    let r_zzz_rcod = data.pop().unwrap_or(0);
    let recursion_available = (0x80 & r_zzz_rcod) == 0x80;
    // TODO the > 16 codes in ResponseCode come from somewhere, (zzz?) need to better understand RFC
    let response_code: ResponseCode = (0x7 & r_zzz_rcod).into();
    let question_count = util::parse_u16(data);
    let answer_count = util::parse_u16(data);
    let name_server_count = util::parse_u16(data);
    let additional_count = util::parse_u16(data);

    Header { id: id, message_type: message_type, op_code: op_code, authoritative: authoritative,
             truncation: truncation, recursion_desired: recursion_desired,
             recursion_available: recursion_available, response_code: response_code,
             question_count: question_count, answer_count: answer_count,
             name_server_count: name_server_count, additional_count: additional_count }
  }
}
