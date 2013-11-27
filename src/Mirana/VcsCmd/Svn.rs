use Shell::e;
use Wrappers::fancy;

///<Summary>
///Svn update
///</Summary>
pub fn svnUpdate() {
    fancy(||{
        e("svn", [&"update"]);
    });
}
