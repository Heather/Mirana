use Model  ::{ App, Repository, Gentoo, MakeCfg
            , pull, push, rebase};

// Modules:
use Misc                ::{toVCS, toTrait};
use VcsCmd::Gentoo      ::{gentooFullUpdate};

use Shell               ::{e};
use Wrappers            ::{λ};

// Stars
use Traits::Vcs;
use extra::{time};

pub fn check(app: &App) {
    for config in (*app).vcs.iter() {
        match config.detector {
            Some(ref detector) => {
                let od = detector.to_owned();
                if (Path::new( od.as_slice() )).exists() {
                    println!("|{:s}:", od);
                    match config.vcs {
                        Some(vcs)       => match toTrait(vcs) {
                            Some(vcs)   => vcs.list(),
                            None        => println!("NO trait for this vcs :(") },
                        None            => println!("No VCS Flavor for this config :(")
                    }
                }
            }, None => ()
        }
    }
}

fn make(cfg: &MakeCfg) {
    let detectorPath = & Path::new( cfg.detector.to_owned() );
    if detectorPath.exists() { 
        λ(||{ for c in cfg.cmd.iter() {
            e(c.as_slice(), []); }
        });
    } else { println!("no {:s} found", cfg.detector);
    }
}

pub fn make_single(app: &App, mk: ~str) {
    match (*app).make.iter().filter_map( |mkCfg| 
        if mkCfg.cfg == mk { Some(mkCfg) }
        else { None }
        ).next() {
            Some(ref cfg) => make(*cfg),
            None => fail!("Non make implementation found")
    };
}

pub fn make_any(app: &App) {
    for cfg in (*app).make.iter() {
        println!(" -> trying {:?}", cfg.cfg)
        make(cfg);
    }
}

pub fn runSync(app: App, repo: Repository, typeFilter: Option<~str>, ncore: uint) {
    let nowt = time::now_utc();
    let nowt_str = nowt.rfc3339();
    for r in repo.remotes.iter().filter(
        |&r| match typeFilter {
            Some(ref rt) => r.t == toVCS(rt.to_owned()),
            None => true
        }) {
        match r.t {
        Gentoo => gentooFullUpdate(repo.loc, ncore),
        _ => {  for b in r.branches.iter() {
                    println!(" [{:s}]  branch: {:s}", nowt_str, *b);
                    match toTrait(r.t) {
                        Some(vcs) => {
                            for action in repo.actions.iter() {
                                match *action {
                                    pull    => vcs.pull_branch(*b),
                                    push    => vcs.pull_branch(*b),
                                    rebase  => vcs.rebase(*b
                                                        , &r.master
                                                        , &r.upstream),
                                    _       => fail!("Non implemented yet")
                                }
                            }
                        }, None => fail!("Non trait implementation found")
                    }
                    match repo.make {
                        Some(ref mk) => { make_single(&app, mk.to_owned());
                        }, None => println!(" [No make]")
                    }
                }
            }
        }
    }
}
