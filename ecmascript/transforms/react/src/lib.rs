pub use self::{
    display_name::display_name,
    jsx::{jsx, Options, Runtime},
    jsx_self::jsx_self,
    jsx_src::jsx_src,
    pure_annotations::pure_annotations,
    refresh::{options::RefreshOptions, refresh},
};
use std::mem;
use swc_common::{chain, comments::Comments, sync::Lrc, Mark, SourceMap};
use swc_ecma_visit::Fold;

mod display_name;
mod jsx;
mod jsx_self;
mod jsx_src;
mod pure_annotations;
mod refresh;

/// `@babel/preset-react`
///
/// Preset for all React plugins.
///
///
/// `top_level_mark` should be [Mark] passed to
/// [swc_ecma_transforms_base::resolver::resolver_with_mark].
pub fn react<C>(
    cm: Lrc<SourceMap>,
    comments: Option<C>,
    mut options: Options,
    top_level_mark: Mark,
) -> impl Fold
where
    C: Comments + Clone,
{
    let Options { development, .. } = options;

    let refresh_options = mem::replace(&mut options.refresh, None);

    chain!(
        jsx_src(development, cm.clone()),
        jsx_self(development),
        refresh(development, refresh_options, cm.clone(), comments.clone()),
        jsx(cm.clone(), comments.clone(), options, top_level_mark),
        display_name(),
        pure_annotations(comments),
    )
}
