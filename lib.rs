#![cfg_attr(not(feature = "std"), no_std)]

use ink_lang as ink;

#[ink::contract(version = "0.1.0")]
mod erc20 {
    use ink_core::storage;
    use ink_core::hash;

    pub type ArtId = u256;
    
    struct Art {
        id: ArtId,
        name: String,
        // Need way to add some kind of hash of image
        creator: AccountId,   
    }

    // Should this be a tuple?
    #[derive(Hash, Eq, PartialEq, Debug)]
    struct AcctArtId {
        accountId: AccountId,
        artId: ArtId
    }

    #[ink(storage)]
    struct Erc20 {
        /// All collections by account
        collections: storage::HashMap<AcctArtId, Art>,
    
        
    }

    #[derive(Encode, Decode, Debug, PartialEq, Eq, Copy, Clone)]
    #[cfg_attr(feature = "std", derive(scale_info::TypeInfo))]
    pub enum Error {
        NotOwner,
        // NotApproved,
        ArtExists,
        ArtNotFound,
        // CannotInsert,
        // CannotRemove,
        // CannotFetchValue,
        // NotAllowed,
    }

    #[ink(event)]
    struct Transfer {
        #[ink(topic)]
        from: Option<AccountId>,
        #[ink(topic)]
        to: Option<AccountId>,
        #[ink(topic)]
        art: Art,
    }

    #[ink(event)]
    struct Create {
        #[link(topic)]
        creator: Option<AccountId>,
        #[link(topic)]
        art: Art,
    }

    #[ink(event)]
    struct Initialize {
        #[link(topic)]
        account: AccountId,
    }

    impl Erc20 {
        // #[ink(constructor)]
        // fn new(&mut self) {
        //     let caller = self.env().caller();
        //     self.painting_count = 0;

        //     self.env()
        //         .emit_event(
        //             Initialize {
        //                 account: caller,
        //             })
        // }
        
        #[ink(constructor)]
        fn new() -> Self {
            Self {
                collections: storage::HashMap::new(),
            }

        #[link(message)]
        fn create_art(&self, name: String) -> Art {
            // First need a way to get a unique id for Art
            let art_id = self.collections.len();
            let caller = self.env().caller();

            // need to check to see if this combination of art and artist already exists
            

            let art = Art {
                id: art_id,
                name,
                creator: caller,
            }

            hash::Wrap()

            let acct_paint_id = AcctPaintId {
                accountId: caller,
                paintingId: painting_id,
            }
            
            match self.collections.entry(acct_paint_id) {
                Entry::Vacant(e) => {e.insert(painting);},
                Entry::Occupied(_) => {println!("This painting account id combo already exists");}
            }

        }

        #[ink(message)]
        fn collection_of(&self, owner: AccountId) -> Vec<Painting> {
            self.collection_or_empty(&owner)
        }

        #[ink(message)]
        fn transfer(&mut self, to: AccountId, painting: Painting) -> bool {
            let from = self.env().caller();
            self.transfer_from_to(from, to, value)
        }

        fn transfer_from_to(&mut self, from: AccountId, to: AccountId, painting: Painting) -> bool {
            // let from_balance = self.balance_of_or_zero(&from);

            // painting.id
            
            // Verify that the from actually owns the painting

            // Remove it from the from
            // Check that the two exists. If it doesn't, add it with a new vec

            let collection = collection_or_empty(&from);

            if collection.is_empty() {
                return false
            }

            if collection.

            if collections.get(from)



            if  {
                return false
            }
            
            

            
            let to_balance = self.balance_of_or_zero(&to);
            


            self.balances.insert(from, from_balance - value);
            self.balances.insert(to, to_balance + value);

            self.env()
                .emit_event(
                    Transfer {
                        from: Some(from),
                        to: Some(to),
                        value,
                    });
            // ACTION: Call `self.env().emit_event` with the `Transfer` event
            //   HINT: Since we use `Option<AccountId>`, you need to wrap accounts in `Some()`
            true
        }

        fn collection_or_empty(&self, owner: &AccountId) -> Vec<Painting> {
            *self.collections.get(owner).unwrap_or(Vec::new())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn new_works() {
            // Should initialize with no values?
            let contract = ArtToken::new();
            assert!(contract.collections.is_empty());
        }
    }
}