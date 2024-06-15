pub mod transducer {
    use std::{fs::File, io::{Read, Write}};
    use rand::{distributions::{Distribution, Uniform}, thread_rng};

    pub struct Transducer {
        states: Vec<Vec<(u8, u8)>>,
        start: u8,
        inverse: Vec<Vec<(u8, u8)>>,
    }
    impl Transducer {
        pub fn spawn() -> Self {
            let mut rng = thread_rng();
            let dist = Uniform::from(0..256);
            let start = dist.sample(&mut rng) as u8;
            let mut state_list = Vec::with_capacity(256);
            let mut inverse_list = Vec::with_capacity(256);
            for i in 0..256 {
                for j in 0..256 {
                    state_list.push((i as u8, j as u8));
                    inverse_list.push((i as u8, j as u8));
                }
            }
            let state: Vec<(u8, u8)> = vec![(0, 0); 256];
            let mut states = vec![state; 256];
            let mut inverses = states.clone();
            while state_list.len() > 0 {
                let i_dist = Uniform::from(0..state_list.len());
                let state_i = i_dist.sample(&mut rng);
                let inverse_i = i_dist.sample(&mut rng);
                let (state, input) = state_list.remove(state_i as usize);
                let (inverse, output) = inverse_list.remove(inverse_i as usize);
                states[state as usize][input as usize] = (inverse, output);
                inverses[inverse as usize][output as usize] = (state, input);
            }
            Transducer { states: states, start: start, inverse: inverses }
        }
        fn to_bytes(&self) -> Vec<u8> {
            let mut bytes = Vec::with_capacity(1 + 256 * 256 * 2);
            bytes.push(self.start);
            for vec in self.states.iter() {
                for (state, change) in vec {
                    bytes.push(*state);
                    bytes.push(*change);
                }
            }
            bytes
        }
        fn from_bytes(b: &Vec<u8>) -> Result<Self, ()> {
            let mut b_iter = b.iter();
            let mut states = vec![vec![(0, 0); 256]; 256];
            let mut inverses = vec![vec![(0, 0); 256]; 256];
            let error = "Not enough bytes to construct transducer";
            let start = *b_iter.next().expect(&error);
            for i in 0..256 {
                for j in 0..256 {
                    let n_state = *b_iter.next().expect(&error);
                    let output = *b_iter.next().expect(&error);
                    states[i][j] = (n_state, output);
                    inverses[n_state as usize][output as usize] = (i as u8, j as u8);
                }
            }
            Ok(Transducer { states: states, start: start, inverse: inverses })
        }
        fn encrypt(&self, message: &Vec<u8>) -> Vec<u8> {
            let mut code = Vec::with_capacity(message.len() + 1);
            let mut state = self.start;
            for b in message.iter() {
                let (n_state, change) = self.states[state as usize][*b as usize];
                state = n_state;
                code.push(change);
            }
            code.push(state);
            code
        }
        fn decrypt(&self, message: &Vec<u8>) -> Result<Vec<u8>, ()> {
            let mut source = Vec::with_capacity(message.len() - 1);
            let mut m_iter = message.iter().rev();
            let mut state = *m_iter.next().expect("Unexpected empty message");
            for b in m_iter {
                let (n_state, original) = self.inverse[state as usize][*b as usize];
                source.insert(0, original);
                state = n_state;
            }
            Ok(source)
        }
        pub fn encrypt_file(&self, path: &str) -> Result<(), ()> {
            let file = File::open(path).expect("Unable to open file with given path");
            let bytes = file.bytes().map(|a| a.unwrap()).collect();
            let cypher = self.encrypt(&bytes);
            let mut file = File::create(path).expect("Unable to open file with given path");
            file.write_all(&cypher).expect("Unable to write to file with cypher");
            Ok(())
        }
        pub fn decrypt_file(&self, path: &str) -> Result<(), ()> {
            let file = File::open(path).expect("Unable to open file with given path");
            let bytes = file.bytes().map(|a| a.unwrap()).collect();
            let cypher = self.decrypt(&bytes);
            let mut file = File::create(path).expect("Unable to open file with given path");
            match cypher {
                Ok(cypher) => {
                    file.write_all(&cypher).expect("Unable to decode cypher in file");
                    Ok(())
                }
                Err(_) => {
                    Err(())
                }
            }
        }
        pub fn save_transducer(&self, path: &str) -> Result<(), ()> {
            let mut file = File::create(path).expect("Unable to open file with given path");
            file.write_all(&self.to_bytes()).expect("Unable to write transducer to file with given path");
            Ok(())
        }
        pub fn load_transducer(path: &str) -> Result<Self, ()> {
            let file = File::open(path).expect("Unable to open file with given path");
            let bytes = &file.bytes().map(|a| a.unwrap()).collect();
            Ok(Self::from_bytes(bytes).unwrap())
        }
    }
}