//
//  ViewController.swift
//  sls-webgpu
//
//  Created by Steven on 11/27/21.
//  Copyright © 2021 Steven Shea. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        // Do any additional setup after loading the view.
        let result = sls_app_make();
        var app: OpaquePointer
        if result.tag == slsAppResult_Ok {
            app = result.ok
            NSLog("app: %i", sls_app_num(app))
            NSLog("cpu count: %i", get_cpu_count())
            sls_app_release(app)
        }
        
        
    }


}

