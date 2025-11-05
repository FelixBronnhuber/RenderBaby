use crate::mvc::View;
use crate::model::FullModel;

pub trait FullViewListener {

}

pub struct FullController<V>
    where V: View<FullModel, dyn FullViewListener>
{
    model: FullModel,
    view: V,
}

impl<V> FullController<V>
    where V: View<FullModel, dyn FullViewListener>
{
    pub fn new(model: FullModel, view: V) -> Self {
        Self { model, view }
    }
}

impl<V> FullViewListener for FullController<V>
    where V: View<FullModel, dyn FullViewListener>
{
    
}
