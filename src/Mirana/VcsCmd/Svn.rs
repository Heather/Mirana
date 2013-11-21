use Shell::e;
use Wrappers::fancy;

///<Summary>
///Svn update
///</Summary>
pub fn svnUpdate() {
    do fancy {
        e("svn", [&"update"]);
    }
}
