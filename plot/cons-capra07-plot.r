
	library( "ggplot2"  )
	library( "ggrepel"  )
	library( "ggthemes" )

	error_bomb <- function( arg ) {
		# Define a error exit function.
		print( "ERROR!", quote = FALSE )

		if ( arg == "no_file_name" ) {
			print( "Input file name and output file name are REQUIRED.", quote = FALSE )
		} else if ( arg == "inadequate_arg" ) {
			print( "Inadequate argument.", quote = FALSE )
		} else if ( arg == "no_input_file_name" ) {
			print( "Input file name is REQUIRED.", quote = FALSE )
		} else if ( arg == "no_output_file_name" ) {
			print( "Output file name is REQUIRED.", quote = FALSE )
		} else if ( "size_data_point" ) {
			print( "-s must be more than 0.0.", quote = FALSE )
		} else if ( "size_title" ) {
			print( "-S must be more than 0.0.", quote = FALSE )
		}

		print( "Program halted!!!", quote = FALSE )

		q( "no" )
	}

	args_list <- commandArgs( trailingOnly = TRUE )
	#print( args_list )

	arg_len <- length( commandArgs( trailingOnly = TRUE ) )
	#print( arg_len )

	arg_i <- NULL # Input filename.
	arg_o <- NULL # Output filename.
	arg_w <- 2500 # Size of width.
	arg_h <- 2250 # Size of height.
	arg_c <- 0    # Colorize.
	arg_s <- 1    # Size of data point.
	arg_t <- 10   # Labeled sites of Top 'n' conservation score ( default top 10 ).
	arg_T <- NULL # Title of the graph.
	arg_S <- 20   # Size of title line.

	if ( arg_len < 4 ) { error_bomb( "no_file_name" ) }

	# Command line parser.
	i <- 1
	while ( i <= arg_len ) {
		arg <- args_list[ i ]
		#print( arg )
		if      ( arg == "-i" ) { arg_i <- args_list[ i + 1 ]               }
		else if ( arg == "-o" ) { arg_o <- args_list[ i + 1 ]               }
		else if ( arg == "-w" ) { arg_w <- as.integer( args_list[ i + 1 ] ) }
		else if ( arg == "-h" ) { arg_h <- as.integer( args_list[ i + 1 ] ) }
		else if ( arg == "-c" ) { arg_c <- as.integer( args_list[ i + 1 ] ) }
		else if ( arg == "-t" ) { arg_t <- as.integer( args_list[ i + 1 ] ) }
		else if ( arg == "-s" ) { arg_s <- as.numeric( args_list[ i + 1 ] ) }
		else if ( arg == "-T" ) { arg_T <- args_list[ i + 1 ]               }
		else if ( arg == "-S" ) { arg_S <- as.numeric( args_list[ i + 1 ] ) }
		else                    { error_bomb( "inadequate_arg" )            }

		i = i + 2
	}

	# Check file name.
	if ( is.null( arg_i ) == TRUE || is.na( arg_i ) == TRUE ) { error_bomb( "no_input_file_name" )  }
	if ( is.null( arg_o ) == TRUE || is.na( arg_o ) == TRUE ) { error_bomb( "no_output_file_name" ) }
	if ( arg_s < 0 )                                          { error_bomb( "size_data_point" )     }
	if ( arg_S < 0 )                                          { error_bomb( "size_title" )          }

	# Show parameters.
	msg_i <- paste( "Input file name         :", arg_i )
	msg_o <- paste( "Output file name        :", arg_o )
	msg_w <- paste( "Size of width / px      :", arg_w )
	msg_h <- paste( "Size of height / px     :", arg_h )
	msg_c <- paste( "Colorize                :", arg_c )
	msg_t <- paste( "Number of Labeled sites :", arg_t )
	msg_s <- paste( "Size of data points     :", arg_s )
	msg_T <- paste( "Title of the graph      :", arg_T )
	msg_S <- paste( "Size of the title       :", arg_S )
	print( msg_i, quote = FALSE )
	print( msg_o, quote = FALSE )
	print( msg_w, quote = FALSE )
	print( msg_h, quote = FALSE )
	print( msg_c, quote = FALSE )
	print( msg_t, quote = FALSE )
	print( msg_s, quote = FALSE )
	print( msg_T, quote = FALSE )
	print( msg_S, quote = FALSE )

	# Set color types.
	# Pale blue -> Strong Blue
	col_low  <- "#74DBEF"
	col_mid  <- "#00D1FF"
	col_high <- "#264E86"
	if ( arg_c == 0 ) {
		# Default color.
		#col_low  <- "#74DBEF"
		#col_mid  <- "#00D1FF"
		#col_high <- "#264E86"
	} else if ( arg_c == 1 ) {
		# Yellow -> Orange -> Red
		col_low  <- "#FFFF00"
		col_mid  <- "#FFA500"
		col_high <- "#FF0000"
	} else if ( arg_c == 2 ) {
		# Green -> Cyan -> Blue
		col_low  <- "#1DE4BD"
		col_mid  <- "#1AC9E6"
		col_high <- "#176BA0"
	} else if ( arg_c == 3 ) {
		# Green -> Yellow -> Red
		col_low  <- "#00FF00"
		col_mid  <- "#FFFF00"
		col_high <- "#FF0000"
	} else if ( arg_c == 4 ) {
		# V a p o r W a v e
		col_low  <- "#FFD700"
		col_mid  <- "#FF00FF"
		col_high <- "#4158D0"
	} else {
		# Do nothing
	}

	# Read the input file.
	dat <- read.table( arg_i, header = TRUE )
	#print( dat )

	# Get TOP n score site number.
	topn <- ifelse( rank( -dat$cons, ties.method = "min" ) <= arg_t, as.character( dat$num ), "" )
	#print( topn )
	#print( order( dat$cons, decreasing = T ) )
	dat <- transform( dat, topn = topn )
	#print( dat )



	# ggplot
	output <- ggplot( data = dat, aes( x = num, y = cons, color = cons, label = topn ) ) +
	#theme_calc() +
	#theme_economist() +
	#theme_economist_white() +
	theme_hc() +
	#theme_light() +
	#theme_stata() +
	#theme_wsj() +
	ggtitle( arg_T ) +
	theme( plot.title = element_text( size = arg_S ) ) +
	xlab( "Site" ) +
	ylab( "Conservation score" ) +
	ylim( 0, 1 ) +
	geom_segment( aes( x = num, xend = num, y = 0, yend = cons ), colour = "gray" ) +
	geom_point( size = arg_s ) +
	geom_label_repel( box.padding = 1, max.overlaps = Inf ) +
	scale_color_gradient2( name = "", low = col_low, mid = col_mid, high = col_high, midpoint = 0.5 )

	# Save the result graph.
	out_name <- paste( arg_o, ".png", sep = "" )

	ggsave( filename = out_name, plot = output, unit = "px", width = arg_w, height = arg_h )

	out_msg <- paste( out_name, "was generated." )
	print( out_msg, quote = FALSE )

	print( "Program completed !!!", quote = FALSE )
