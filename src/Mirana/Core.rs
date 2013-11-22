use Model  ::{ App, Repository, Gentoo
            , pull, push, rebase};

// Modules:
use Misc                ::{toVCS, toTrait};
use VcsCmd::Gentoo      ::{gentooFullUpdate};
use Shell               ::{e};

// Stars
use Traits::Vcs;
use extra::{time};

fn make(app: &App, mk: ~str) {
    match (*app).make.iter().filter_map( |mkCfg| 
        if mkCfg.cfg == mk { Some(mkCfg) }
        else { None }
        ).next() {
        Some(ref cfg) => {
            let detectorPath = & Path::new( (*cfg).detector.to_owned() );
            if detectorPath.exists() { e((*cfg).detector, []);
            } else { println!("no {:s} found", (*cfg).detector);
            }
        }, None => { fail!("Non make implementation found") }
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
                                    pull    => vcs.pull(*b),
                                    push    => vcs.push(*b),
                                    rebase  => vcs.rebase(*b
                                                        , &r.master
                                                        , &r.upstream),
                                    _       => fail!("Non implemented yet")
                                }
                            }
                        }, None => fail!("Non trait implementation found")
                    }
                    match repo.make {
                        Some(ref mk) => { make(&app, mk.to_owned());
                        }, None => println(" [No make]")
                    }
                }
            }
        }
    }
}
