use pdf::content::Operation;
use pdf::primitive::Primitive;

#[derive(Debug, Clone)]
pub struct PrimitiveParser<'src, F>
    where F: FnMut()
{
    operations: std::slice::Iter<'src, Operation>,
    extractor: F,
}

impl<'src, F> PrimitiveParser<'src, F>
    where F: FnMut() {
    pub fn get_primitives(operations: &[Operation], extractor: F) -> impl Iterator<Item=Vec<&[Primitive]>>
        where F: FnMut()
    {
        PrimitiveParser {
            operations: operations.iter(),
            extractor,
        }
    }

    pub fn parse_to_string(primitives: Vec<&[Primitive]>) -> String {
        let mut text: Vec<String> = Vec::new();

        for &primitive in primitives.iter() {
            for obj in primitive.iter() {
                if let Primitive::String(pdf_string) = obj {
                    if let Some(string) = pdf_string.clone().into_string().ok() {
                        text.push(string);
                    }
                }
            }
        }
        text.join("")
    }
}

impl<'src, F> Iterator for PrimitiveParser<'src, F>
    where F: FnMut()
{
    type Item = Vec<&'src [Primitive]>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(Operation { operator, operands }) = self.operations.next() {
            if let ("TJ", _primitives) = (operator.as_str(), operands.as_slice()) {
                return Some(_primitives
                    .into_iter()
                    .map(|primitive| primitive.as_array().unwrap())
                    .collect::<Self::Item>());
            }
        }
        None
    }
}
