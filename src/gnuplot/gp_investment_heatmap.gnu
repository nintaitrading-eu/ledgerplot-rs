# This plots a heatmap over 3 accounts with 10 commodities max.
#
# Example dat file:
# ,account01,account02,account03
# assets:current\\\_assets:stock:xy\\\_abc,3,0,0,0,0
# assets:current\\\_assets:stock:yz\\\_def,2,0,0,0,0
# assets:current\\\_assets:etf:yz\\\_met,0,0,1,0,0
# assets:current\\\_assets:etf:yz\\\_bat,0,0,2,0,0
# assets:current\\\_assets:stock:gh\\\_xyz,0,3,0,0,0
# assets:current\\\_assets:stock:xx\\\_aaa,0,3,0,0,0
# assets:current\\\_assets:etf:yz\\\_idx,4,0,1,0,0
# tbd,0,0,0,0,0
# tbd,0,0,0,0,0
# tbd,0,0,0,0,0

set terminal pngcairo size 1900, 1080 enhanced font "Inconsolata,12"
set output 'investment_heatmap.png'
unset key
set view map scale 1
set style data lines

set xtics border in scale 0,0 mirror norotate offset character 0, -0.2, 0 autojustify
set xtics norangelimit 
set xtics ()
set xrange [ -0.500000 : 2.50000 ] noreverse nowriteback # Change the y-value here, to add more accounts.

set ytics border in scale 0,0 mirror norotate autojustify
set ytics norangelimit 
set ytics ()
set yrange [ -0.500000 : 9.50000 ] noreverse nowriteback

set cbtics border in scale 0,0 mirror norotate offset character -7.0, 5.0, 0 autojustify
set cbtics norangelimit 0.00000,1 ,5.00000

set cblabel "Value range (EUR)" 
set cbrange [ 0.00000 : 6.00000 ] noreverse nowriteback

set palette positive nops_allcF maxcolors 6 gamma 1.5 color model RGB
set palette defined (0 "#FFFFFF", 1 "#EEDDEE", 2 "#D9B2D9", 3 "#C488C4", 4 "#A850A8", 5 "#800080") 
set colorbox vertical origin screen 0.9, 0.2 size screen 0.05, 0.6 front noinvert bdefault

set title "Heat map investments" font "Inconsolata,18"
set datafile separator comma
plot '/tmp/ledgerplot/investment_heatmap.dat' matrix rowheaders columnheaders using 1:2:3 with image
set datafile separator
