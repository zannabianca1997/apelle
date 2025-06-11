use apelle_configs_dtos::QueueUserAction;

fn main() {
    println!("CREATE TYPE queue_user_action AS ENUM(");
    for action in QueueUserAction::iter() {
        println!("  '{}',", action);
    }
    println!(");");
}
