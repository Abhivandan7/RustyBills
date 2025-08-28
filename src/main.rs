use std::io::stdin;
// use colored::Colorize;

struct Item{
    _id: u32,
    _name: String,
    _price: f32,
}

struct ItemList{
    _items: Vec<Item>,
    _total_expense: f32,
}

impl ItemList {
    fn add_item(&mut self, _item: Item){
        self._total_expense += _item._price;
        self._items.push(_item);

        println!("______________________________________\n");
        println!("Added Item to the list");
    }
    
    fn del_item(&mut self, _item_id: usize){
        let _item: Item = self._items.remove(_item_id-1);
        self._total_expense -= _item._price;

        println!("______________________________________\n");
        println!("Item Removed from the list")
    }
    
    fn disp_item(&self){
        println!("+{:-^47}+", "Items");
        for _item in self._items.iter(){
            println!("| {:>2} | {:<30} | $ {:>5} |",_item._id, _item._name, _item._price)
        }
        println!("+{:-^47}+", '-');

    }

    fn print_total_expense(&self){
        println!("The Total Expense:");
        println!("$ {}",self._total_expense);
    }
    
}


fn main() {

    let mut _item_list: ItemList = ItemList{
        _items: Vec::new(),
        _total_expense: 0.0,
    };

    println!(
        "\n--------------------------- {} ---------------------------\n",
        "Welcome to the Expense Tracker App"
    );

    let mut _item_counter: u32 = 1;

    loop {
        let mut _choice: String = String::new();
        println!(
            "Choose an option to perform: \n{} \n{} \n{} \n{} \n{}\n",
            "1. Add Item", "2. Delete Item", "3. Display Items", "4. Calculate Total Expense", "5. Exit");

        stdin()
            .read_line(&mut _choice)
            .expect("Error in reading input");

        let _choice: u8 = _choice
            .trim()
            .parse()
            .expect("Invalid Entry: Expected a number!");
        
        println!("______________________________________\n");

        match _choice {
            1 => {
                let mut _item_name: String = String::new();
                let mut _item_price: String = String::new();
                println!("Enter the item details:");
                println!("Name:");
                stdin()
                    .read_line(&mut _item_name)
                    .expect("Error reading item name!");
                let _item_name: String = _item_name.trim().parse().expect("Error in parsing item name");
                println!("\nPrice:");
                stdin()
                    .read_line(&mut _item_price)
                    .expect("Error reading item price!");
                let _item_price: f32 = _item_price.trim().parse().expect("Error in parsing item price");

                // Create a Item Objet and call add_item function

                let _item: Item = Item{
                    _id: _item_counter,
                    _name: _item_name,
                    _price: _item_price,
                };

                _item_list.add_item(_item);
                _item_counter += 1;

            }
            2 => {
                let mut _id: String = String::new();
                _item_list.disp_item();
                println!("Enter the id of the element to remove:");
                stdin().read_line(&mut _id).expect("Error in reading the id");
                let _id: usize = _id.trim().parse().expect("Error in parsing the id");

                _item_list.del_item(_id);
            }
            3 => {
                _item_list.disp_item()
            }
            4 => {
                _item_list.print_total_expense();
            }
            5 => {
                println!("Quitting...\n");
                break;
            }
            _ => {
                break;
            }
        }
        println!("______________________________________\n");
    }
}
