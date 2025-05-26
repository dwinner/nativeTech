fn dump_sorted<T>(mut collection: T)
where
   T: Sort + IntoIterator,
   T::Item: std::fmt::Debug,
{
   // 'Sort' trait bound
   collection.sort();

   // 'IntoIterator' trait bound
   for item in collection
   {
      // 'Debug' trait bound
      println!("{:?}", item);
   }
}
