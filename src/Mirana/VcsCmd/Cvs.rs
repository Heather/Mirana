use Shell::e;
use Wrappers::fancy;

///<Summary>
///Cvs update
///</Summary>
pub fn cvsUpdate() {
    do fancy {
        e("cvs", [&"update"]);
    }
}
