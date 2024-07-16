Generate a key:
cargo run -- -g {key path} e.g cargo run -- -g key.txt

Encrypt a file:
cargo run -- -e {key path} {file path} e.g cargo run -- -e key.txt text.txt

Decrypt a file:
cargo run -- -d {key path} {file path} e.g cargo run -- -d key.txt text.txt

How it works:
This encryption algorithm uses a finite state transducer to encypt the source.It essentially 
takes each byte and converts it to another byte, then based on that original byte, gets 
taken to the next state. When it reaches the end, it appends a byte id representing the final 
state. On decryption, it uses the key to generate an inverse of the encryption finite state 
transducer uses the id at the end to find the state it starts from. Then its works backward
to get the original text. The transducer has exactly 256 states, one for each possible byte, 
and each state has 256 output edges and 256 input edges, also for each byte. This ensures that
you can always create an inverse without there being ambiguity in what state to go to next in
the decryption.

Pros:
-The key length is always fixed at the memory required to store all edges and states and won't
arbitrarily grow in size.
-Technically you could start in any state since any movement from any place is reversable, so
encrypting the same file twice with the same key would yield different results.
-Without knowing anything about the source message, the cyphertext would be undeciferable since
there could be multiple "right" answers.

Cons
-The key length is always fixed at the memory required to store all edges and states and cannot be
smaller.
-Given enough information about the source text you could map out the state relationship of the key,
and could likely get a partial or full decrypt. Although a full decrypt would likely need lots of 
source text that is statistically random enough to map every state and edge.
-Encrypted text still keeps the same length as its original plus the extra byte for state storage

Possible Improvements:
-Instead of having each state represented by one byte, it could be 2 or an arbitrary amount, at the 
downside of increasing key size 256x for each extra byte which could grow quite fast, but so would the 
required amount of source text to decrypt without key.
-Similiarily, we can allow each state to have a varied amount of inputs, if it is greater than 256,
we can have an input byte instead be two output bytes, or an arbitrary amount if needed, instead of one.
This doesn't increase key size as fast as the above change, but could cause the encrypted text to be
much longer than its source.
