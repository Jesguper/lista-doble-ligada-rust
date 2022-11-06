use std::{
    //cell son contenedores mutables compartidos que tienes un objeto T que es la referencia al contenedor ya sean mutables o inmutables
    //RefCell es la ubicacion en la memoria donde se encuentra la referencia con reglas de prestamo dinamico
    cell::RefCell,
    //rc son punteros de conteo de referencia de subprocesos individuales
    //Rf el puntero de conteo de referencia
    //Weak es una version de Rf al cual para acceder al valor se usa upgrade que devuelve un Option<RC<T>>, para llamarlo se usa Rc::downgrade.
    rc::{Rc, Weak},
};


//Copy se usa para 'copiar' una variable a otra y poder seguir utilizando la variable original
//Option se refiere a que puede tener alguno de esos valores y usa las funciones Some (Determina un valor) y None
pub struct Node<T: Copy>{
    pub value: T,
    pub next: Option<Rc<RefCell<Node<T>>>>,
    pub prev: Option<Weak<RefCell<Node<T>>>>,
}

//impl 'implementa' una funcionalidad para la estructura Node
impl<T: Copy> Node<T>{
    //Crea el nodo y se retorna a si mismo como Node
    pub fn new(value: T) -> Self {
        Node{
            value,
            next: None,
            prev: None,
        }
    }
}

//convierte el Node<T> a un valor de Option
impl<T: Copy> From<Node<T>> for Option<Rc<RefCell<Node<T>>>>{
    fn from(node: Node<T>) -> Self{
        //Alguna opcion de referencia
        Some(Rc::new(RefCell::new(node)))
    }
}

//NodePtr es para las posibles opciones de puntero que tiene la lista
type NodePtr<T> = Rc<RefCell<Node<T>>>;

//La estructura de la lista 
pub struct List<T: Copy> {
    head: Option<NodePtr<T>>,
    tail: Option<NodePtr<T>>,
}

//Implementa las funcionalidades de la lista
impl<T:Copy> List<T> {
    //Crea una nueva lista vacia y la retorna
    pub fn new() -> Self{
        List{
            head:None,
            tail:None
        }
    }

    //Implementa la funcion de push_front
    //push_front(), inserta un nodo al inicio de la lista.
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

    //implementa la funcion de push_back
    //push_back(), inserta un nodo al final de la lista.
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

    //funcion de pop_frot, devuelve un Option
    //pop_front(), elimina un nodo al inicio de la lista, regresa el objeto eliminado.
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

    //implementa la funcion de pop_back, devuelve un Option
    //pop_back(), elimina el nodo del final de la lista, regresa el objeto eliminado.
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
}

fn main(){

}