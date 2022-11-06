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

`rc::Rc`: Es un puntero de conteo de referencia propietario, es el puntero que se usara para apuntara la cabeza ("Head") de la lista.

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

### Estructura de la lista

```rust
pub struct List<T: Copy> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
}
```

```rust
type NodePtr<T> = Rc<RefCell<Node<T>>>;
```

NodePtr<T> unicamente es la abreviacion de todos los valores que puede tener el Option de la lista 


## Funciones 

Al iniciar con la implementación de las funciones de la lista, se comienza el uso de variables y referencias, por lo que, es importarnte indicar que hay dos tipos tantto de variables como de referencias y son las siguientes:
- No mutables: Una varaible o referencia no mutable **NO** podra cambiar su valor, por lo que, es muy importante tomar en cuanta si en un futuro se necesitara cambiar dicho valor o no. 
- Mutables: El valor de la variable o referencia puede cambiar.

```rust
// No mutables
let var
&referencia

// Mutables
let mut var
&mut referencia
```

### push_front()
Es la función encargada de inserta un nodo al inicio de la lista.

```rust
    pub fn push_front(&mut self, value: T){
        //self es la lista
        
        //Crea un nuevo nodo mutable
        let mut node = Node::new(value);
        
        //take() toma el valor de Option de la lista que lo invoca
        match &mut self.head.take(){
            //En caso de que el valor de head sea None
            None => {
                //head toma el valor del nodo
                self.head = node.into();
                //Se copia el valor de la cabeza a la cola de la lista
                self.tail = self.head.clone();
            },
            //en caso de que head tenga un valor
            Some(current_head) => {
                //Copia el valor actual de head y se lo pone al siguiente nodo
                node.next = Some(current_head.clone());
                //Se agrega el nodo al inicio de la lista
                self.head = node.into();
                // toma el valor de la referencia de head, y si tiene algo continua
                if let Some(h) = &self.head {
                    //El valor de head previo toma la referencia debil de head actual
                    current_head.borrow_mut().prev = Some(Rc::downgrade(&h));
                }
            }
        }
    }
```

### push_back()
Es la función encargada de inserta un nodo al final de la lista.

```rust
    pub fn push_back(&mut self, value: T){
        //Crea un nuevo nodo mutable
        let mut node = Node::new(value);

        //self es la lista que invoca el metodo
        
        //toma el valor actual de la cola de la lista (tail)
        match &mut self.tail.take(){
            None => {
                //head toma el valor del nodo
                self.head = node.into();
                //Se copia el valor de la cabeza a la cola de la lista
                self.tail = self.head.clone();
            },
            //En caso de que encuentre un valor en tail
            Some(current_tail) => {
                //el nodo anterior toma la referencia debil de la cola actual
                node.prev = Some(Rc::downgrade(&current_tail));
                //Se Agrega el nodo al final de la lista
                self.tail = node.into();
                //Se recorre el valor de tail a la siguiente posicion
                current_tail.borrow_mut().next = self.tail.clone();
            }
        }
    }
```

### pop_front()
Es la función encargada de eliminar el nodo al inicio de la lista, regresa el objeto eliminado.
```rust
    pub fn pop_front(&mut self) -> Option<T>{
        //Revisa el valor de head de la lista
        match &mut self.head.take() {
            //Si no encuentra nada, no hace nada
            None => None,
            //Si encuentra algo
            Some(head) =>{
                //Recupera el valor envuelto (wrappeed) de head y lo deja muteable
                let mut head = head.borrow_mut();
                //Toma el valor next de head y lo guarda en una variable inmutable
                let next = head.next.take();
                //Revisa el valor de next
                match next {
                    //Si esta vacio
                    None => {
                        //toma el valor de tail de la lista
                        self.tail.take();
                    },
                    //Si encuentra un valor en next
                    Some(next) => {
                        //le asigna none al valor prev 
                        next.borrow_mut().prev = None;
                        //El valor de head de la lista sera el valor de next
                        self.head= Some(next);
                    }
                };
                //Devuelve el nodo eliminado
                Some(head.value)
            }
        }
    }
```

### pop_back()
Es la función encargada de eliminar el nodo del final de la lista, regresa el objeto eliminado.

```rust
    pub fn pop_back(&mut self) -> Option<T>{
        //Recupera el valor de tail y lo revisa
        match &mut self.tail.take(){
            //Si no encutra nada, no hace nada
            None => None,
            //Si encuentra algo
            Some(tail)=>{
                //Toma prestado el valor desenvuelto de tail
                let mut tail = tail.borrow_mut();
                //Toma el valor de la tail previa
                let prev = tail.prev.take();
                //Revisa si hay una cola previa
                match prev {
                    //Si no hay, se recupera el valor de head de la lista
                    None => {
                        self.head.take();
                    },
                    //Si tiene un valor
                    Some(prev) => {
                        //accede al valor de prev y lo guarda como variable inmutable
                        let prev = prev.upgrade();
                        //Si tiene un valor
                        if let Some(prev) = prev {
                            //toma prestado el valor next y le pone None
                            prev.borrow_mut().next = None;
                            //al valor tail de la lista se le asigna el valor de prev
                            self.tail = Some(prev);
                        }
                    }
                };
                //Devuelve el nodo eliminado
                Some(tail.value)
            }
        }
    }
```
