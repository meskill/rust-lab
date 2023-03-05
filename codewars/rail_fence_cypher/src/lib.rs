fn single_rail<Handler>(size: usize, num_rails: usize, rail_num: usize, mut handler: Handler)
where
    Handler: FnMut(usize),
{
    let mut n = rail_num;
    let mut down_line = true;
    let mut from_down_to_up_modifier = 2 * (num_rails - rail_num - 1);
    let mut from_up_to_down_modifier = 2 * rail_num;

    if from_down_to_up_modifier == 0 {
        from_down_to_up_modifier = from_up_to_down_modifier;
    }

    if from_up_to_down_modifier == 0 {
        from_up_to_down_modifier = from_down_to_up_modifier;
    }

    while n < size {
        handler(n);

        n += if down_line {
            from_down_to_up_modifier
        } else {
            from_up_to_down_modifier
        };

        down_line = !down_line;
    }
}

pub fn encode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let chars: Vec<_> = text.chars().collect();
    let size = chars.len();
    let mut result = "".to_owned();

    for rail_num in 0..num_rails {
        single_rail(size, num_rails, rail_num, |n| {
            result.push(chars[n]);
        });
    }

    result
}

pub fn decode_rail_fence_cipher(text: &str, num_rails: usize) -> String {
    let size = text.chars().count();
    let mut chars = text.chars();
    let mut result: Vec<char> = vec![' '; size];

    for rail_num in 0..num_rails {
        single_rail(size, num_rails, rail_num, |n| {
            result[n] = chars.next().unwrap();
        });
    }

    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_tests() {
        assert_eq!(encode_rail_fence_cipher("abcdefghi", 2), "acegibdfh");
        assert_eq!(encode_rail_fence_cipher("abcdefghi", 3), "aeibdfhcg");
        assert_eq!(encode_rail_fence_cipher("abcdefghi", 4), "agbfhceid");
        assert_eq!(encode_rail_fence_cipher("abcdefghi", 5), "aibhcgdfe");

        assert_eq!(decode_rail_fence_cipher("acegibdfh", 2), "abcdefghi");
        assert_eq!(decode_rail_fence_cipher("aeibdfhcg", 3), "abcdefghi");
        assert_eq!(decode_rail_fence_cipher("agbfhceid", 4), "abcdefghi");
        assert_eq!(decode_rail_fence_cipher("aibhcgdfe", 5), "abcdefghi");

        assert_eq!(
            encode_rail_fence_cipher("WEAREDISCOVEREDFLEEATONCE", 3),
            "WECRLTEERDSOEEFEAOCAIVDEN"
        );
        assert_eq!(
            decode_rail_fence_cipher("WECRLTEERDSOEEFEAOCAIVDEN", 3),
            "WEAREDISCOVEREDFLEEATONCE"
        );
        assert_eq!(
            encode_rail_fence_cipher("Hello, World!", 3),
            "Hoo!el,Wrdl l"
        );
        assert_eq!(
            decode_rail_fence_cipher("Hoo!el,Wrdl l", 3),
            "Hello, World!"
        );
    }
}
