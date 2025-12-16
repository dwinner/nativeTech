fn main()
{
   /*let mut server=NetworkServer::default();
   while !server.done(){
      // Get data to a new vector
      let data: Vec<u8> = read_data_from_socket();
      if let Err(e) = server.process(&data){
         log::error!("Failed to process data: {:?}", e);
      }
   }*/
}

/// Type - length - value from data stream
#[derive(Clone, Debug)]
pub struct Tlv /* <'a> */
{
   pub type_code: u8,
   pub value: Vec<u8>, /* data which a heap owns
                        *pub value: &'a [u8], */
}

// some local error type
pub type Error = &'static str;

/// Retrieving next TLV from `input`
/// and returning rest of unhandled data
pub fn get_next_tlv(input: &[u8]) -> Result<(Tlv, &[u8]), Error>
{
   if input.len() < 2
   {
      return Err("too short for a TLV");
   }

   // In TLV size of each element T and L is 1 byte
   let type_code = input[0];
   let len = input[1] as usize;
   if 2 + len > input.len()
   {
      return Err("TLV is longer than remaining data");
   }

   let tlv = Tlv {
      type_code,
      value: input[2..2 + len].to_vec(),
      // ref. to input data fragment
      // value: &input[2..2+len],
   };

   Ok((tlv, &input[2 + len..]))
}

pub struct NetworkServer /* <'a> */
{
   // the last max sized message
   max_size: Option<Tlv /* <'a> */>,
}

/// Code of message type set-maximum-size
const SET_MAX_SIZE: u8 = 0x01;

impl NetworkServer /* <'a> */
{
   pub fn process(&mut self, mut data: & /* 'a */ [u8]) -> Result<(), Error>
   {
      while !data.is_empty()
      {
         let (tlv, rest) = get_next_tlv(data)?;
         match tlv.type_code
         {
            SET_MAX_SIZE =>
            {
               // saving the last message `SET_MAX_SIZE`
               self.max_size = Some(tlv);
            }
            // handling of other types of messages
            _ => return Err("unknown message type"),
         }

         data = rest; // handling the rest of data in the next iteration
      }

      Ok(())
   }
}
