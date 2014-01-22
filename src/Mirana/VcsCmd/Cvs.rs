use Shell::e;
use Wrappers::λ;

///<Summary>
///Cvs update
///</Summary>
pub fn cvsUpdate() {
    λ(||{ e("cvs", [&"update"]); });
}
