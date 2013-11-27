use Shell::e;
use Wrappers::fancy;

///<Summary>
///Cvs update
///</Summary>
pub fn cvsUpdate() {
    fancy(||{
        e("cvs", [&"update"]);
    });
}
