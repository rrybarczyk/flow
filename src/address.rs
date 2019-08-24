use crate::common::*;

#[derive(Debug, PartialEq)]
pub(crate) struct Address {
    pubkey: String,
    ip: SocketAddr,
}

impl FromStr for Address {
    type Err = Error;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let idx = match text.find('@') {
            Some(i) => i,
            None => {
                return Err(Error::NoAt {
                    bad_addr: text.to_owned(),
                })
            }
        };
        let pubkey = text[..idx].to_owned();
        let addr = &text[idx + 1..];
        let ip = SocketAddr::from_str(addr).map_err(|addr_parse| Error::AddrParse {
            addr_parse,
            bad_addr: addr.to_string(),
        })?;
        Ok(Address { pubkey, ip })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_good() -> Result<(), Error> {
        let want = Address {
            pubkey: String::from("pub"),
            ip: SocketAddr::from_str("127.0.0.1:8080").unwrap(),
        };
        let my_str = "pub@127.0.0.1:8080";
        let have = Address::from_str(my_str)?;
        assert_eq!(have, want);
        Ok(())
    }

    #[test]
    fn no_at() {
        assert_eq!(
            Address::from_str("invalid"),
            Err(Error::NoAt {
                bad_addr: "invalid".to_owned(),
            })
        );
    }
}
