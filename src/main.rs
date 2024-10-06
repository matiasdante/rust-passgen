use rand::distributions::Alphanumeric; // Para obtener caracteres alfanuméricos aleatorios
use rand::Rng; // Para generar números aleatorios
use std::fs::OpenOptions;
use std::io; // Para manejar la entrada del usuario
use std::iter; // Para trabajar con iteradores // Para manejar archivos

fn main() {
    // Solicitamos al usuario que ingrese la longitud de la contraseña
    println!("Introduce la longitud de la contraseña:");

    let mut length = String::new(); // Creamos una variable mutable para almacenar la entrada del usuario

    io::stdin()
        .read_line(&mut length) // Leemos la entrada
        .expect("Error al leer la entrada"); // Manejamos posibles errores en la lectura

    let length: usize = match length.trim().parse() {
        // Convertimos la entrada en un número
        Ok(num) => num, // Si la conversión fue exitosa, usamos el valor
        Err(_) => {
            println!("Por favor, introduce un número válido.");
            return;
        } // Si la conversión falla, mostramos un mensaje de error y terminamos el programa
    };

    // Generamos la contraseña aleatoria
    let password: String = generate_password(length); // Llamamos a la función que genera la contraseña

    // Mostramos la contraseña generada
    println!("Tu contraseña generada es: {}", password);

    // Guardamos la contraseña en el archivo
    save_password_to_file(&password);
}

// Función que genera una contraseña aleatoria
fn generate_password(length: usize) -> String {
    // Usamos iteradores para generar una secuencia de caracteres al azar de la longitud especificada
    let password: String = iter::repeat_with(|| rand::thread_rng().sample(Alphanumeric))
        .take(length) // Tomamos el número de caracteres especificado por el usuario
        .map(char::from) // Convertimos los números aleatorios en caracteres
        .collect(); // Recolectamos los caracteres en una cadena

    password // Devolvemos la contraseña generada
}

// Función para guardar la contraseña en un archivo de texto
fn save_password_to_file(password: &str) {
    // Abrimos el archivo `passwords.txt` en modo append, para agregar nuevas contraseñas sin sobrescribir las existentes
    let mut file = OpenOptions::new()
        .append(true) // Agregar contenido al final del archivo
        .create(true) // Si el archivo no existe, lo creará
        .open("passwords.txt") // Especificamos el archivo donde se guardarán las contraseñas
        .expect("No se pudo abrir el archivo");

    // Escribimos la contraseña generada en una nueva línea en el archivo
    use std::io::Write; // Importamos el trait Write para poder usar el método `write_all`
    writeln!(file, "{}", password).expect("No se pudo escribir en el archivo");
}
