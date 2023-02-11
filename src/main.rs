use uuid::Uuid;

#[derive(Clone,Debug)]
struct Todo {
    id: String,
    task: String,
    completed: bool,
}

fn generate_uuid() -> String {
    let uuid: Uuid = Uuid::new_v4();
    uuid.to_string()
}

fn prompt_user() -> String {
    let mut input: String = String::new();

    println!("\n{}\n{}\n{}\n{}\n{}", "Please choose an option:", "[a]dd new task", "[s]how tasks", "[u]pdate status", "[q]uit");
    std::io::stdin().read_line(&mut input).unwrap();

    let return_value: String = input.trim().to_string().to_lowercase();


    if return_value != "a" && return_value != "s" && return_value != "u" && return_value != "q" {
        println!("Invalid input. Please try again.");
        prompt_user();
    }

    return_value
}

fn add_task() -> Todo {
    let mut input: String = String::new();

    println!("Enter a task: ");
    std::io::stdin().read_line(&mut input).unwrap();


    let task: String = input.trim().to_string();

    let todo: Todo = Todo {
        id: generate_uuid(),
        task,
        completed: false,
    };

    todo
}

fn show_todos(todos: &Vec<Todo>) {
    if todos.len() == 0 {
        println!("No tasks found.");
        return;
    }

    for todo in todos {
        let completed: String = if todo.completed { "Completed" } else { "Not completed" }.to_string();
        println!("{} - {} - {}", todo.id, todo.task, completed);
    }
}

fn update_status(todos: &mut Vec<Todo>) {
    println!("Which status would you like to update? (Type the long-ass UUID)");
    show_todos(todos);

    let mut long_ass_uuid_input: String = String::new();

    std::io::stdin().read_line(&mut long_ass_uuid_input).unwrap();

    let long_ass_uuid = long_ass_uuid_input.trim().to_string();

    let mut index: Option<usize> = None;
    for i in 0..todos.len() {
        if todos[i].id == long_ass_uuid {
            index = Some(i);
            break;
        }
    }

    let result: Option<&Todo> = todos.iter().find(|todo: &&Todo| todo.id == long_ass_uuid_input.trim().to_string());

    if let Some(todo) = result {
        let index = todos.iter().position(|x: &Todo| x.id == todo.id).unwrap();
        let mut todo = &mut todos[index];
        todo.completed = !todo.completed;
        println!("Task with ID {} is now {}", todo.id, if todo.completed { "completed" } else { "incomplete" });
    } else {
        println!("No task found with ID: {}", long_ass_uuid);
    }
}




fn main() {

    let run: bool = true;
    let mut todos: Vec<Todo> = Vec::new();

    while run {

        let option: String = prompt_user();

        if option == "q" {
            break;
        } else if option == "s" {
            show_todos(&todos);
            continue;
        } else if option == "u" {
            update_status(&mut todos);
            continue;
        } else if option == "a" {
            let todo: Todo = add_task();
            todos.push(todo);
        } else {
            println!("Invalid input. Please try again.");
            continue;
        }

    }

}


