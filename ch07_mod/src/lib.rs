pub  use back_of_house::breakfast_mod::Breakfast;

mod back_of_house;

fn eat_at_restaurant(){
    let meal = Breakfast::summer("light");
}