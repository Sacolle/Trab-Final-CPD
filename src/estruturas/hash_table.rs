struct HashTable<K,T>
where K: PartialEq
{
	vec: Vec<Link<K,T>>,
	hash: Box<dyn Fn(&K, usize)->usize>,
	size: usize
}
type Link<K,T> = Option<Box<Node<K,T>>>;
struct Node<K,T>{
	key: K,
	value: T,
	next: Link<K,T>
}

impl<K,T> HashTable<K,T>
where K: PartialEq
{
	pub fn new(
		size:usize,
		hash_function: impl for<'a>Fn(&'a K,usize)->usize + 'static
	)->Self
	{
		let mut vec = Vec::with_capacity(size);
		vec.resize_with(size, ||None);

		let hash = Box::new(hash_function);
		HashTable { vec, hash, size }
	}

	pub fn insert(&mut self,key: K,value: T){
		let idx = (self.hash)(&key,self.size);
		let head = &mut self.vec[idx];
		
		let new_node = Box::new(
			Node{
				key,
				value,
				next: head.take()
			}
		);
		*head = Some(new_node);
	}
	pub fn get(&self,key: K)->Option<&T>{
		let idx = (self.hash)(&key,self.size);
		let mut head = &self.vec[idx];

		while let Some(node) = head{
			if node.key == key{
				return Some(&node.value);
			}
			head = &node.next;
		}
		None
	}
	/*
    pub fn remove(&mut self,key: K){
		let idx = (self.hash)(&key,self.size);
		let head = &mut self.vec[idx];
        
		head.take().map(|node| {
            *head = node.next;
        });
    }
	*/
}

impl<K,T> Drop for HashTable<K,T>
where K: PartialEq
{
    fn drop(&mut self){
		for lists in self.vec.iter_mut(){
			//println!("Dropped list");
			let mut cur_link = lists.take();
			while let Some(mut boxed_node) = cur_link {
				cur_link = boxed_node.next.take();
			}
		}
    }
}


#[cfg(test)]
#[allow(unused_imports)]
mod tests{
	use super::*;
	#[test]
	fn working_table(){
		let mut hash_table:HashTable<usize, usize> = HashTable::new(4, |x,y|x%y);
		
		hash_table.insert(1, 1);
		hash_table.insert(2, 2);
		hash_table.insert(3, 3);
		hash_table.insert(4, 0);
		hash_table.insert(5, 5);

		let hit = hash_table.get(5);
		let miss = hash_table.get(7);
		assert_eq!(hit,Some(&5));
		assert_eq!(miss,None);
	}
}