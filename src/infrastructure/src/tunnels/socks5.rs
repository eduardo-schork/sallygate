//! The smallest SOCKS5 server side that browsers and `curl` use: no login, CONNECT only.

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

const VERSION: u8 = 5;
const NO_AUTHENTICATION: u8 = 0;
const NO_ACCEPTABLE_METHOD: u8 = 0xFF;
const CONNECT: u8 = 1;
const IPV4: u8 = 1;
const DOMAIN: u8 = 3;
const IPV6: u8 = 4;
const SUCCEEDED: u8 = 0;
const GENERAL_FAILURE: u8 = 1;
const COMMAND_NOT_SUPPORTED: u8 = 7;

fn refused(reason: &str) -> std::io::Error {
    std::io::Error::new(std::io::ErrorKind::InvalidData, reason.to_owned())
}

/// Reads the client's greeting and CONNECT request, and returns where it wants to go.
pub(super) async fn read_request(
    client: &mut (impl AsyncRead + AsyncWrite + Unpin),
) -> std::io::Result<(String, u16)> {
    let [version, method_count] = read_array(client).await?;
    if version != VERSION {
        return Err(refused("not a SOCKS5 client"));
    }
    let mut methods = vec![0; usize::from(method_count)];
    client.read_exact(&mut methods).await?;
    if !methods.contains(&NO_AUTHENTICATION) {
        client.write_all(&[VERSION, NO_ACCEPTABLE_METHOD]).await?;
        return Err(refused("the client wants a login, which is not offered"));
    }
    client.write_all(&[VERSION, NO_AUTHENTICATION]).await?;

    let [_, command, _, address_type] = read_array(client).await?;
    if command != CONNECT {
        client
            .write_all(&[VERSION, COMMAND_NOT_SUPPORTED, 0, IPV4, 0, 0, 0, 0, 0, 0])
            .await?;
        return Err(refused("only CONNECT is supported"));
    }
    let host = match address_type {
        IPV4 => std::net::Ipv4Addr::from(read_array::<4>(client).await?).to_string(),
        IPV6 => std::net::Ipv6Addr::from(read_array::<16>(client).await?).to_string(),
        DOMAIN => {
            let [length] = read_array(client).await?;
            let mut name = vec![0; usize::from(length)];
            client.read_exact(&mut name).await?;
            String::from_utf8(name).map_err(|_| refused("the host name is not text"))?
        }
        _ => return Err(refused("unknown address type")),
    };
    let port = u16::from_be_bytes(read_array(client).await?);
    Ok((host, port))
}

/// Tells the client whether the connection was made. The bound address is not shared.
pub(super) async fn reply(
    client: &mut (impl AsyncWrite + Unpin),
    succeeded: bool,
) -> std::io::Result<()> {
    let code = if succeeded {
        SUCCEEDED
    } else {
        GENERAL_FAILURE
    };
    client
        .write_all(&[VERSION, code, 0, IPV4, 0, 0, 0, 0, 0, 0])
        .await
}

async fn read_array<const N: usize>(
    reader: &mut (impl AsyncRead + Unpin),
) -> std::io::Result<[u8; N]> {
    let mut bytes = [0; N];
    reader.read_exact(&mut bytes).await?;
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn reads_a_connect_to_a_host_name() {
        let (mut client, mut server) = tokio::io::duplex(64);
        let request = tokio::spawn(async move { read_request(&mut server).await });

        client.write_all(&[5, 1, 0]).await.unwrap();
        let mut choice = [0; 2];
        client.read_exact(&mut choice).await.unwrap();
        client
            .write_all(&[
                5, 1, 0, 3, 9, b'e', b'x', b'a', b'm', b'p', b'l', b'e', b'.', b'c', 0, 80,
            ])
            .await
            .unwrap();

        assert_eq!(choice, [5, 0]);
        assert_eq!(
            request.await.unwrap().unwrap(),
            ("example.c".to_owned(), 80)
        );
    }
}
