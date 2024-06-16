pub mod transducer {
    use std::{fs::File, io::{Read, Write}};
    use rand::{distributions::{Distribution, Uniform}, thread_rng};

    pub struct Transducer {
        /* First dimension represents current state, Second represents current input
        The tuple has the next state as first entry and the new byte as the second entry */
        states: Vec<Vec<(u8, u8)>>, 
        /* The exact inverses of the states attribute. This state and next state are swapped. This input and new byte are swapped
        This makes finding the previous state for decrypting not have to search the whole 2-D array
        Both states and inverse are digraphs that compliment each other */
        inverse: Vec<Vec<(u8, u8)>>,
        // The chosen start state of the message
        start: u8,
    }
    impl Transducer {
        // Randomly creates a transducer
        pub fn spawn() -> Self {
            // Create random number generator
            let mut rng = thread_rng();
            // Dist for obtaining the start state
            let dist = Uniform::from(0..256);
            let start = dist.sample(&mut rng) as u8;
            /* Create a list of state-byte pairs and the inverse
            We do this so each state in the diagraph has 256 edges exactly with each having a unique byte */
            let mut state_list = Vec::with_capacity(256);
            let mut inverse_list = Vec::with_capacity(256);
            for i in 0..256 {
                for j in 0..256 {
                    // Create every comnbination of state-input, and inverse-output
                    state_list.push((i as u8, j as u8));
                    inverse_list.push((i as u8, j as u8));
                }
            }
            // Initialize states and its compliment
            let state: Vec<(u8, u8)> = vec![(0, 0); 256];
            let mut states = vec![state; 256];
            let mut inverses = states.clone();
            // While there are still digraph pairs to be made, make them
            while state_list.len() > 0 {
                // Create a uniform dist for every entry in the lists
                let i_dist = Uniform::from(0..state_list.len());
                // Get pair positions
                let state_i = i_dist.sample(&mut rng);
                let inverse_i = i_dist.sample(&mut rng);
                // Remove these pairs from the lists
                let (state, input) = state_list.remove(state_i as usize);
                let (inverse, output) = inverse_list.remove(inverse_i as usize);
                // Connect them in the digraphs
                states[state as usize][input as usize] = (inverse, output);
                inverses[inverse as usize][output as usize] = (state, input);
            }
            // Return completed transducer
            Transducer { states: states, start: start, inverse: inverses }
        }
        // Converts the transducer to a its byte form
        fn to_bytes(&self) -> Vec<u8> {
            // 256 rows * 256 columns * 2 bytes in each entry plus one start state
            let mut bytes = Vec::with_capacity(1 + 256 * 256 * 2);
            // Push start
            bytes.push(self.start);
            for vec in self.states.iter() {
                for (state, change) in vec {
                    /* Push its state then what the byte is changed to
                    We don't need to save the inverse since all the information is encoded here */ 
                    bytes.push(*state);
                    bytes.push(*change);
                }
            }
            bytes
        }
        // Takes a byte vector and constructs the transducer from it
        fn from_bytes(b: &Vec<u8>) -> Result<Self, ()> {
            let mut b_iter = b.iter();
            // Technically we don't need to do it this way and can construct the 2-D array as we get the appropiate values
            let mut states = vec![vec![(0, 0); 256]; 256];
            let mut inverses = vec![vec![(0, 0); 256]; 256];
            // Get the start. Any correct format of the transducer will have enough iterated bytes
            let start = match b_iter.next() {
                Some(s) => *s,
                None => return Err(()),
            };
            for i in 0..256 {
                for j in 0..256 {
                    // Get bytes
                    let n_state = match b_iter.next() {
                        Some(n) => *n,
                        None => return Err(()),
                    };
                    let output = match b_iter.next() {
                        Some(o) => *o,
                        None => return Err(()),
                    };
                    // Store the states and the inverse
                    states[i][j] = (n_state, output);
                    inverses[n_state as usize][output as usize] = (i as u8, j as u8);
                }
            }
            Ok(Transducer { states: states, start: start, inverse: inverses })
        }
        // Takes a byte vector and converts it using the transducer
        fn encrypt(&self, message: &Vec<u8>) -> Vec<u8> {
            // Extra space for the final state since decrypting without it is impossible
            let mut code = Vec::with_capacity(message.len() + 1);
            let mut state = self.start;
            for b in message.iter() {
                // Get next state and the change to the byte
                let (n_state, change) = self.states[state as usize][*b as usize];
                // Update state and push the new byte
                state = n_state;
                code.push(change);
            }
            // Push final state
            code.push(state);
            code
        }
        // Decrypt a byte vector using this transducer
        fn decrypt(&self, message: &Vec<u8>) -> Result<Vec<u8>, ()> {
            // We don't need the final state byte so we can reduce size by 1
            let mut source = Vec::with_capacity(message.len() - 1);
            // Reverse so we go back to front
            let mut m_iter = message.iter().rev();
            // Even an empty source message will have a byte in the cypher so we shouldn't expect this
            let mut state = match m_iter.next() {
                Some(s) => *s,
                None => return Err(()),
            };
            for b in m_iter {
                // This is why the inverse exists. Otherwise we would have to search for the right spot
                let (n_state, original) = self.inverse[state as usize][*b as usize];
                // Insert original byte and get new state
                source.insert(0, original);
                state = n_state;
            }
            Ok(source)
        }
        // Encrypt a file from path with this transducer
        pub fn encrypt_file(&self, path: &str) -> Result<(), ()> {
            let file = match File::open(path) {
                Ok(f) => f,
                Err(_) => return Err(()),
            };
            // There is probably a better way to do this, but it works
            let bytes = file.bytes().map(|a| a.unwrap()).collect();
            // Get cypher
            let cypher = self.encrypt(&bytes);
            let mut file = match File::create(path) {
                Ok(f) => f,
                Err(_) => return Err(()),
            };
            // Write over original text
            match file.write_all(&cypher) {
                Err(_) =>  Err(()),
                _ => Ok(())
            }
        }
        // Decrypt a file from path with this transducer
        pub fn decrypt_file(&self, path: &str) -> Result<(), ()> {
            let file = match File::open(path) {
                Ok(f) => f,
                Err(_) => return Err(()),
            };
            // There is probably a better way to do this, but it works
            let bytes = file.bytes().map(|a| a.unwrap()).collect();
            // Get source
            let source = match self.decrypt(&bytes) {
                Ok(s) => s,
                Err(_) => return Err(()),
            };
            let mut file = match File::create(path) {
                Ok(f) => f,
                Err(_) => return Err(()),
            };
            // Write source
            match file.write_all(&source) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
        // Save the transducer to a file at this path
        pub fn save_transducer(&self, path: &str) -> Result<(), ()> {
            let mut file = match File::create(path) {
                Ok(f) => f,
                Err(_) => return Err(()),
            };
            // Get byte form and write it
            match file.write_all(&self.to_bytes()) {
                Ok(_) => Ok(()),
                Err(_) => Err(()),
            }
        }
        // Load the transducer from a file at this path
        pub fn load_transducer(path: &str) -> Result<Self, ()> {
            let file = match File::open(path) {
                Ok(f) => f,
                Err(_) => return Err(()),
            };
            // There is probably a better way to do this, but it works
            let bytes = &file.bytes().map(|a| a.unwrap()).collect();
            // Construct and return
            Self::from_bytes(bytes) 
        }
    }
}