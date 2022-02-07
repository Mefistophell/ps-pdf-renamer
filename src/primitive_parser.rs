use pdf::content::{Content, Operation};
use pdf::primitive::Primitive;

#[derive(Debug, Clone)]
pub struct PrimitiveParser<'a> {
    operations: std::slice::Iter<'a, Operation>,
}

impl<'a> PrimitiveParser<'a> {
    pub fn parse(content: &'a Content) -> impl Iterator<Item=String> + 'a {
        PrimitiveParser {
            operations: content.operations.iter(),
        }
    }

    fn retrieve_string() -> fn(&Primitive) -> String {
        |primitive: &Primitive| {
            let mut string = String::new();
            let primitives = primitive.as_array().unwrap();

            for primitive in primitives.iter() {
                if let Primitive::String(pdf_string) = primitive {
                    if let Some(symbol) = pdf_string.clone().into_string().ok() {
                        string.push_str(&symbol);
                    }
                }
            }

            return string;
        }
    }
}

impl Iterator for PrimitiveParser<'_> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Operation { operator, operands }) = self.operations.next() {
            if let ("TJ", _primitives) = (operator.as_str(), operands.as_slice()) {
                return Some(_primitives
                    .into_iter()
                    .map(PrimitiveParser::retrieve_string())
                    .collect::<Self::Item>());
            }
        }
        None
    }
}
