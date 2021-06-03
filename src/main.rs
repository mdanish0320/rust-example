#[derive(Debug, Clone)]
struct Product {
  name: String,
  code: i32
}

// struct ProductList(Vec<Product>);
pub struct ProductList {
  products: Vec<Product>
}

impl ProductList {
    
    fn new() -> Self {
    return ProductList {
      products: Vec::new()
    };
}

    fn add_product(&mut self, name: String, code: i32){
      let new_product = Product{
        name: name,
        code: code
      };

      self.products.push(new_product);
    }

    fn show_all(&self) -> Vec<Product>{
      return self.products.clone();
    }

    fn get_product_by_id(&self, code: i32) -> Option<Product>{
      
      for st in self.products.iter() {
        if st.code == code {
          return Some(st.clone());
        }
      }
      
      None
    }

    fn remove_product(&mut self, code: i32){
      for (pos, e) in self.products.iter().enumerate() {
        if e.code == code {
          self.products.remove(pos);
          break;
        } 
      }
    }
}

fn main(){
  
  let mut product_list = ProductList::new();
  product_list.add_product(String::from("product-1"), 1);
  product_list.add_product(String::from("product-2"), 2);

  println!("{:?}", product_list.get_product_by_id(2));
  println!("{:?}", product_list.show_all());
  product_list.remove_product(2);
  println!("{:?}", product_list.get_product_by_id(2));
}


