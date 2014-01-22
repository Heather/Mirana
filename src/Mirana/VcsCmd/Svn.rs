use Shell::e;
use Wrappers::λ;

///<Summary>
///Svn update
///</Summary>
pub fn svnUpdate() {
    λ(||{
        e("svn", [&"update"]);
    });
}
