#!/usr/bin/expect -f

spawn cargo run --bin synacor-challenge

#give ourselfs some time to attach the debugger
#sleep 10

global expect_out
set output ""                

set inputs { "go doorway\r" 
			 "go north\r"
			 "go north\r"
			 "go bridge\r"
			 "go continue\r"
			 "go down\r"
			 "go east\r"
			 "take empty lantern\r"
			 "go west\r"
			 "go west\r"
			 "go passage\r"
			 "go ladder\r"
			 "go west\r"
			 "go south\r"
			 "go north\r"
			 "take can\r"
			 "go west\r"
			 "go ladder\r"
			 "use can\r"
			 "use lantern\r"
			 "go darkness\r"
			 "go continue\r"
			 "go west\r"
			 "go west\r"
			 "go west\r"
			 "go west\r"
			 "go north\r"
			 "use lit lantern\r"
			 "take red coin\r"
			 "go north\r"
			 "go west\r"
			 "take blue coin\r"
			 "go up\r"
			 "take shiny coin\r"
			 "go down\r"
			 "go east\r"
			 "go east\r"
			 "take concave coin\r"
			 "go down\r"
			 "take corroded coin\r"
			 "go up\r"
			 "go west\r"
			 "use blue coin\r"
			 "use red coin\r"
			 "use shiny coin\r"
			 "use concave coin\r"
			 "use corroded coin\r"
			 "go north\r"
			 "take teleporter\r"
			 "fix teleporter\r"
			 "use teleporter\r"
			 "go north\r"
			 "go north\r"
			 "go north\r"
			 "go north\r"
			 "go north\r"
			 "go north\r"
			 "go north\r"
			 "go north\r"
			 "go north\r"
			 "take orb\r"
			 "go north\r"
			 "go east\r"
			 "go east\r"
			 "go north\r"
			 "go west\r"
			 "go south\r"
			 "go east\r"
			 "go east\r"
			 "go west\r"
			 "go north\r"
			 "go north\r"
			 "go east\r"
			 "go vault\r"
			 "take mirror\r"
			 "use mirror\r"
}

foreach input $inputs {
	expect {
		"do\?" {
			send $input
			append output $expect_out(buffer)
		}
	}
}

send_user $output
interact
