/*
* Hashtable implementada com a resolução de confitos com encadeamento fechado
* Usando uma lista encadeada para isso
*/
pub struct HashTable<K,T>
where K: PartialEq
{
	vec: Vec<Link<K,T>>,
	hash: Box<dyn Fn(&K, usize)->usize>,
	size: usize
}
type Link<K,T> = Option<Box<Node<K,T>>>;
pub struct Node<K,T>{
	key: K,
	value: T,
	next: Link<K,T>
}

impl<K,T> HashTable<K,T>
where K: PartialEq
{
	pub fn new(size:usize,hash_function: impl for<'a>Fn(&'a K,usize)->usize + 'static)->Self {
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
	//retorna uma referencia imutável a um elemento
	pub fn get(&self,key: &K)->Option<&T>{
		let idx = (self.hash)(key,self.size);
		let mut head = &self.vec[idx];

		while let Some(node) = head{
			if &node.key == key{
				return Some(&node.value);
			}
			head = &node.next;
		}
		None
	}
	//retorna uma referencia mutável a um elemento
	pub fn get_mut(&mut self,key: &K)->Option<&mut T>{
		let idx = (self.hash)(key,self.size);
		let mut head = &mut self.vec[idx];

		while let Some(node) = head{
			if &node.key == key{
				return Some(&mut node.value);
			}
			head = &mut node.next;
		}
		None
	}
	//função helper que retorna um vetor com todos os elementos da table 
	pub fn all(&self)->Vec<&T>{
		let mut res = Vec::new();
		for lists in self.vec.iter(){
			let mut cur_link = lists.as_ref();
			while let Some(b_node) = cur_link {
				res.push(&b_node.value);
				cur_link = b_node.next.as_ref();
			}
		}
		res
	}
	//função helper para ver quantos elementos tem na table
	pub fn _entries(&self)->usize{
		let mut res = 0;
		for lists in self.vec.iter(){
			let mut cur_link = lists.as_ref();
			while let Some(b_node) = cur_link {
				res += 1;
				cur_link = b_node.next.as_ref();
			}
		}
		res
	}
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
pub mod utils{
	use crate::io::parser::Player;
	use super::HashTable;
	//funções de hash para a table
	pub fn hash_usize(key:&usize,size:usize)->usize{
		key%size
	}
	pub fn hash_string(key:&String,size:usize)->usize{
		// p = 31
		let mut hash:usize = 0;
		for byte in key.as_bytes(){
			hash = (31 * hash + *byte as usize) % size
		}
		hash
	}
	//função helper para converter o raiting acumulado do player para o raiting médio
	pub fn divide_raiting(table: &mut HashTable<usize,Player>){
		for _link in table.vec.iter_mut(){
			let mut link = _link;
			while let Some(node) = link{
				node.value.rating /= node.value.count as f64;
				link = &mut node.next;
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

		let hit = hash_table.get(&5);
		let miss = hash_table.get(&7);
		assert_eq!(hit,Some(&5));
		assert_eq!(miss,None);
	}
}