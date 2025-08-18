// TODO: Flesh out the `WeekTemperatures` struct and its method implementations to pass the tests.

pub struct WeekTemperatures {
    /* TODO */,
}

pub enum Weekday {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
    Sunday,
}

impl WeekTemperatures {
    pub fn new() -> Self {
      /* TODO */  }
    }

    pub fn get_temperature(&self, day: Weekday) -> Option<i32> {
     /* TODO */ex]
    }

    pub fn set_temperature(&mut self, day: Weekday, temperature: i32) {
     /* TODO */e);
    }
/* TODO: Create weekday2index method which converts a Weekday enum variant into its corresponding zero-based index. */ }
}
