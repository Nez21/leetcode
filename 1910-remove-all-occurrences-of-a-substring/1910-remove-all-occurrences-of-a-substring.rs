impl Solution {
    pub fn remove_occurrences(s: String, part: String) -> String {
        let part_bytes = part.into_bytes();

        String::from_utf8(
            s.chars()
                .into_iter()
                .fold(Vec::<u8>::new(), |mut bytes, ch| {
                    bytes.push(ch as u8);

                    if bytes.len() >= part_bytes.len()
                        && bytes[bytes.len() - part_bytes.len()..bytes.len()] == part_bytes
                    {
                        bytes.truncate(bytes.len() - part_bytes.len());
                    }

                    bytes
                }),
        )
        .unwrap()
    }
}