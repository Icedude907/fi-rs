
/// Enhanced if statement functions.
/// 1. Ternary: `fi!(expr, true_case, false_case)`
/// 2. If-let chain check to replace `matches!`: `fi!(let Some(a) = x && let Some(b) = y && a == b)`
#[macro_export]
macro_rules! fi {
    // Ternary operator
    ($condition:expr, $true_case:expr, $false_case:expr) => {
        {if $condition { $true_case } else { $false_case }}
    };
    // if-let pattern match (enhanced `matches!()`)
    ($($cond:tt)*) => {
        {if $($cond)* { true } else { false }}
    };
}
