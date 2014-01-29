use Wrappers::λ;

use extra::getopts::{Opt};

pub fn print_usage(program: &str, _opts: &[Opt], nix: bool) {
    λ(||{
        println!("Usage: {} [options]", program);
        println!("
        -h --help\tUsage
        -v --version\tDisplay version

        -j --jobs\tThreads

        check\t Display current repository vcs
        
        commit\t commit changes
        pull\t pull changes
        push\t push changes in any vcs
        
        make\t build current project or specified one
        sync\t perform sync of specified project

        -l --list\tPretty print repositories in sync
        -d\t\tDelete repo from configuration
        -a\t\tAdd repo to configuration

        -e --edit\t\tEdit repo configuration

            --add\t\tAdd something to repo configuration
            --delete\tDelete something from repo configuration

        -s --sync\t\tSync config
        -r --remote\t\tSpecify remote
        -u --upstream\t\tSpecify upstream repository
        -m --master\t\tSpecify upstream master branch
        -b --branch\t\tBranch of adding / editing repo or filtering type
        -x --exec\t\tActual action for repository (pull, push, rebase)
        -t --type\t\tType of adding / editing repo or filtering type");
        if nix {
            println!("        -g --gentoo\t\tSync Gentoo-x86");
        } else {
            println!("
            Stone word backfire with vengeance
                    Hopeless divine intervention
                    
                                Leader, where's the peace you pursue
        Can't let any more follow you
                Teach to bleach the stains of your guilt
            Envy of moral free lives built
                            Live with the torment that they live through
                            
                            Your sins will only rest on you

            ");
        }
    });
}