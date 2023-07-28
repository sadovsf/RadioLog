use std::collections::HashMap;

use turbosql::{Turbosql, execute};




pub struct DataStore<T> {
    list: Vec<T>,
    map:  HashMap<i64, usize>,

    data_version: u32
}

pub trait DataStoreTrait {
    fn set_id(&mut self, id :i64);
    fn get_id(&self) -> i64;
}


impl<T> DataStore<T>
    where T: Turbosql + Default + DataStoreTrait
{
    pub fn get_version(&self) -> u32 {
        self.data_version
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.list.iter()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn from(data :Vec<T>) -> Self {
        let mut inst = DataStore {
            list: Vec::new(),
            map: HashMap::new(),
            data_version: 1,
        };

        for item in data {
            inst.add_internal(item);
        }

        inst
    }

    pub fn edit(&mut self, item :T) -> Result<(), turbosql::Error> {
        item.update()?;

        let id = item.get_id();
        let index = self.map.get(&id).expect("Failed to edit item");
        self.list[*index] = item;

        self.data_version += 1;
        Ok(())
    }


    fn add_internal(&mut self, item :T) {
        let item_id = item.get_id();

        self.list.push(item);
        self.map.insert(item_id, self.list.len() - 1);
    }

    pub fn add(&mut self, item :T) -> Result<(), turbosql::Error> {
        let item_id = item.insert()?;
        let mut item = item;
        item.set_id(item_id);
        self.add_internal(item);
        self.data_version += 1;
        Ok(())
    }

    pub fn remove(&mut self, id :i64) -> Result<(), turbosql::Error> {
        let index = self.map.remove(&id).expect("Failed to remove item");
        self.list.remove(index);
        execute!("DELETE FROM logentry WHERE rowid = ?", id)?;
        self.data_version += 1;
        Ok(())
    }

    pub fn get(&self, id :i64) -> Option<&T> {
        let index = self.map.get(&id)?;
        self.list.get(*index)
    }

    pub fn get_by_index(&self, index :usize) -> Option<&T> {
        self.list.get(index)
    }

    pub fn find_index_of(&self, id :i64) -> Option<usize> {
        self.map.get(&id).map(|i| *i)
    }
}