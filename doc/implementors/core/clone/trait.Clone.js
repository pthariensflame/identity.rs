(function() {var implementors = {};
implementors["identity"] = ["impl&lt;'param&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.LiToTy.html' title='identity::lift::LiToTy'>LiToTy</a>&lt;'param&gt;","impl&lt;F:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.LiFunToTyFun.html' title='identity::lift::LiFunToTyFun'>LiFunToTyFun</a>&lt;F&gt;","impl&lt;F:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.TyFunToLiFun.html' title='identity::lift::TyFunToLiFun'>TyFunToLiFun</a>&lt;F&gt;","impl&lt;TFa:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>,&nbsp;TFb:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.Compose.html' title='identity::lift::Compose'>Compose</a>&lt;TFa,&nbsp;TFb&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.Id.html' title='identity::lift::Id'>Id</a>","impl&lt;X:&nbsp;?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.Const.html' title='identity::lift::Const'>Const</a>&lt;X&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.ConstTyFun.html' title='identity::lift::ConstTyFun'>ConstTyFun</a>","impl&lt;F:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>,&nbsp;G:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.Ap.html' title='identity::lift::Ap'>Ap</a>&lt;F,&nbsp;G&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.ApTyFun0.html' title='identity::lift::ApTyFun0'>ApTyFun0</a>","impl&lt;F:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.ApTyFun1.html' title='identity::lift::ApTyFun1'>ApTyFun1</a>&lt;F&gt;","impl&lt;F:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.Flip.html' title='identity::lift::Flip'>Flip</a>&lt;F&gt;","impl&lt;F:&nbsp;<a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a>,&nbsp;ParamB:&nbsp;?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.Flipped.html' title='identity::lift::Flipped'>Flipped</a>&lt;F,&nbsp;ParamB&gt;","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.FlipTyFun.html' title='identity::lift::FlipTyFun'>FlipTyFun</a>","impl <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.Nil.html' title='identity::lift::Nil'>Nil</a>","impl&lt;A:&nbsp;?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a>,&nbsp;As:&nbsp;<a class='trait' href='identity/lift/trait.TyList.html' title='identity::lift::TyList'>TyList</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/lift/struct.Cons.html' title='identity::lift::Cons'>Cons</a>&lt;A,&nbsp;As&gt;","impl&lt;A:&nbsp;?<a class='trait' href='https://doc.rust-lang.org/nightly/core/marker/trait.Sized.html' title='core::marker::Sized'>Sized</a>&gt; <a class='trait' href='https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html' title='core::clone::Clone'>Clone</a> for <a class='struct' href='identity/struct.Refl.html' title='identity::Refl'>Refl</a>&lt;A&gt;",];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
