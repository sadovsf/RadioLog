use std::{collections::HashMap, cell::{RefCell, RefMut}};
use rusqlite::Connection;

use crate::database::{Database, DBObjectSerializable, DBSchemaObject};




pub struct DataStore<'a, T> {
    db: &'a RefCell<Database>,

    list: Vec<T>,
    map:  HashMap<i64, usize>,

    data_version: u32
}

pub trait DataStoreTrait {
    fn set_id(&mut self, id :i64);
    fn get_id(&self) -> i64;
}


impl<'a, T> DataStore<'a, T>
    where T: DBSchemaObject + DBObjectSerializable + DataStoreTrait
{
    fn get_db(&mut self) -> RefMut<'_, Database> {
        self.db.borrow_mut()
    }

    pub fn get_version(&self) -> u32 {
        self.data_version
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.list.iter()
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn new(db :&'a RefCell<Database>) -> Result<Self, rusqlite::Error> {
        let mut inst = DataStore {
            db,
            list: Vec::new(),
            map: HashMap::new(),
            data_version: 1,
        };

        let mut db = db.borrow_mut();
        db.register_type::<T>()?;

        for it in db.select_all::<T>()? {
            inst.add_internal(it);
        }

        Ok(inst)
    }

    pub fn edit(&mut self, item :T) -> Result<(), rusqlite::Error> {
        self.get_db().update(&item)?;

        let id = item.get_id();
        let index = self.map.get(&id).expect("Failed to edit item");
        self.list[*index] = item;

        self.data_version = self.data_version.wrapping_add(1);
        Ok(())
    }


    fn add_internal(&mut self, item :T) {
        let item_id = item.get_id();

        self.list.push(item);
        self.map.insert(item_id, self.list.len() - 1);
    }

    pub fn add(&mut self, item :T) -> Result<(), rusqlite::Error> {
        let mut item = item;
        self.get_db().insert(&mut item)?;

        self.add_internal(item);
        self.data_version = self.data_version.wrapping_add(1);
        Ok(())
    }

    pub fn remove(&mut self, id :i64) -> Result<(), rusqlite::Error> {
        let index = self.map.remove(&id).expect("Failed to remove item");
        let instance = self.list.remove(index);

        self.get_db().delete(&instance)?;

        self.data_version = self.data_version.wrapping_add(1);
        Ok(())
    }

    pub fn get(&self, id :i64) -> Option<&T> {
        let index = self.map.get(&id)?;
        self.list.get(*index)
    }

    pub fn get_where<P>(&mut self, where_clause :&str, params :P) -> Result<Vec<T>, rusqlite::Error>
    where
        P: rusqlite::Params
    {
        self.get_db().select_where(where_clause, params)
    }

    pub fn get_by_index(&self, index :usize) -> Option<&T> {
        self.list.get(index)
    }

    pub fn find_index_of(&self, id :i64) -> Option<usize> {
        self.map.get(&id).map(|i| *i)
    }
}