# Lista doblemente ligada en Rust

El objetivo de este repositorio es explicar las políticas de propiedad y mutabilidad de la lista doblemente ligada implementada por Thomas Hollow en el siguiente [video](https://www.youtube.com/watch?v=k0cL6K28SL0).

### Primero se importa la biblioteca estandar de la cual se usan las librerias de cell y rc.
```rust
use std::{
    cell::RefCell,
    rc::{Rc, Weak},
};
```
`cell::RefCell`: Es una ubicación de memoria mutable con reglas de préstamo dinámicas, el decir que es mutable significa que puede cambiar, esta ubicación sera la encargada de almacenar los nodos.

`rc::Rc`: Es un puntero de conteo de referencia, es el puntero que se usara para apuntara la cabeza ("Head") de la lista.

`rc::Weak`: Es un puntero derivado de Rc el cual se encarga de apuntar a una referencia no propietaria, normalmente se usan esto puntero para apuntar a una referecnia "fuerte" (Rc) con el fin de evitar que se hagan referencias circulares, en la implementación estos punteros serán usados para la cola ("tail") de la pila, este puntero hace uso de dos funciones.

- `upgrade()`: Se usa para obtener la referencia al puntero fuerte (Rc).
- `Rc::downgrade()`: Se usa para crear el puntero debil ("Weak") que apuntara al puntero fuerte (Rc).

### Estructura del nodo
```rust
pub struct Node<T: Copy>{
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}
```
En la estructura del Nodo hay dos puntos importantes a destacar que diferencian a Rst de otros lenguajes de programación:
- `Option`: Indica que puede tomar los distintos valores que pone entre <> y cuando es usado solo toma dos valores:
  - `None`: No tiene ningun valor.
  - `Some()`: Contiene alguno de sus posibles valores.
- `Copy`: Es un formato que copia el valor que recibe de una variable sin afectarla.

Para poder agregarle una función a una estructura se usa `impl`.
```rust
impl<T: Copy> Node<T>{
    //Crea el nodo y se retorna a si mismo
    pub fn new(value: T) -> Self {
        Node{
            value,
            next: None,
            prev: None,
        }
    }
}
```
