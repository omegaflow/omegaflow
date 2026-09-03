      Program Extract9
c
c      This program is designed to extract and write to file for editing 
c      the five card data set associated with each asteroid or comet number 
c      provided by the user.
c
c      The user will input the desired number.
c
c      The output file will be assigned by the program and the user will be
c       provided the name for access.
c
c       The file that will be created will consist of 28 fields per number
c       requested and will occupy 395 bytes.
c
c
c..............................................................................
c       Designed and Implemented by: Ravenel N. Wimberly
c                                    User Technology Associates
c                                    Solar Systems Dynamics Group
c                                    Asteroid and Comet Studies Group
c                                    Navigation & Mission Design Section
c                                    Systems Division
c                                    ................................
c                                        May       15, 2002
c                                        December   1, 2003
c                                    ................................
c..............................................................................
c
c
c  .. Define variables ..
      real*8         epoch, calepo, percal, perjd, VAL
      real*8         a, e, ri, om, rnode, rmo, pyr, q
      integer        no, nobs, IBIAS1, IBIAS2
      integer*4      isub, i1, i2, l1, l2
      character*4    equnox
      character*6    sptype
      character*8    iref
      character*9    darc
      character*13   desig
      character*12   SMA
      character*18   astnam, ENDPT, beginp
      character*24   comnam
      character*41   comnt1, blank1
      character*49   comnt3, blank3
      character*80   comnt2, blank2
      character*105  filler
c
      blank1 = '                                         '
      blank2 = blank1//blank1(1:39)
      blank3 = blank1//'        '
c
c  .. Open files for processing
c
      open(10,file='editfile',status='new',access='sequential')
      open(20,file='dastcom9.',status='old',access='direct',
     1     form='unformatted',recl=395)
c
c  .. Clear The Screen
      write(*,'(//////////////////////////)')
c   
c  .. Read Dastcom9 Header Information  
c   
         read (20,rec=1) IBIAS1,ENDPT,epoch,calepo,equnox,beginp,
     1                    filler,IBIAS2,comnt1,comnt2  
c   
c      Decode & Display End Point Object Numbers
c
       read (endpt,'(3i6)') iast1,iast2,icomet
       write(6,*) ' *********** DASTCOM9 CONTAINS ***************'
       write(6,*) ' Numbered Asteroids:         1 to ',iast1
       write(6,*) ' Un-Numbered Asteroids: 100001 to ',iast2
       write(6,*) ' Comets:                400001 to ',icomet
       write(6,*) ' *********************************************'
c
c     Pause and wait for user response
      write(*,'(//////////)')
c
c .. Display Current Header Information 
c   
      write(6,'(/,25h  Dastcom9 Was Created : ,a2,2(1h:,a2),1x,
     1          2a2,2(1h:,a2))') (comnt1(i1:i1+1),i1=1,14,2)
      write(6,'(//,20x,9h:UPDATES:,//)')  
      write(6,'(6x,9hReplace  ,a2,2(1h:,a2),2a2,2(1h:,a2))')
     1          (comnt1(i1:i1+1),i1=17,30,2)
      write(6,'(6x,9hUnum2num ,a2,2(1h:,a2),2a2,2(1h:,a2))')
     1          (comnt2(i1:i1+1),i1=1,14,2) 
      write(6,'(6x,9hUpdate1  ,a2,2(1h:,a2),2a2,2(1h:,a2))')
     1          (comnt2(i1:i1+1),i1=17,30,2)
      write(6,'(6x,9hUpdate2  ,a2,2(1h:,a2),2a2,2(1h:,a2))')
     1          (comnt2(i1:i1+1),i1=33,46,2)
      write(6,'(6x,9hCupdate  ,a2,2(1h:,a2),2a2,2(1h:,a2))')
     1          (comnt2(i1:i1+1),i1=49,62,2)
      write(6,'(6x,9hGetascom ,a2,2(1h:,a2),2a2,2(1h:,a2),//)') 
     1          (comnt2(i1:i1+1),i1=65,78,2)
c   
      call system('PAUSE')
c   
c  .. Clear The Screen
      write(*,'(//////////////////////////)')
c  .. Prompt User for Input
10    write(*,*)     ' ....To Terminate Input a -1,0 at the Prompt ....'
      write(*,*)     ' Example: 400001,400035 or 400001,0 or -1,0'
      write(*,*) ' ..Please Input The Number Range Desired..  -------> '
      read (*,*,end=10) i1,i2
c
c  .. Check I1 for Termination
      if(i1.le.0) go to 100
c  .. Set Record Bias
      isub = 0
      IF(I1.GE.1)      ISUB = 1
      if(i1.ge.100001) isub = -IBIAS1
      if(i1.ge.400001) isub = -IBIAS2
      l1 = i1 + isub
      l2 = i2 + isub
      if(l2.le.l1) l2=l1
c
c  .. Process Request for Comet Information
      if(i1.ge.400001) then
        do 20 mx=l1,l2
c                        Comets
         read (20,rec=mx)   no,comnam,epoch,calepo,equnox,iref,desig,
     1                      rmo,rnode,om,ri,e,VaL,
     2                      perjd,percal,q,gm,rad,a1,a2,
     3                      bvt,b10n,pyr,darc,comnt3,comnt2
c  .. Set All 'NULL' Characters to 'BLANKS'
        comnam(1:1) = ' '
        if(ichar(comnt1(1:1)).eq.0) comnt3 = blank3
        if(ichar(comnt2(1:1)).eq.0) comnt2 = blank2
c
c    Convert Double Precision Semi-Major Axis (a) into a string (12 chars wide)
        call fmtdp(val,.true.,sma)
c
        write(10,'(i6,a,2f11.1,1x,a,1x,a8,1x,a13/4x,4f12.7,f12.9,a/4x,
     *   f15.7,f17.7,f13.9,1x,f7.4,f7.2,2f8.4/2x,f5.2,f6.2,f8.2,1x,
     *   a,a/a)') 
     *                      no,comnam,epoch,calepo,equnox,iref,desig,
     *                      rmo,rnode,om,ri,e,SMa,
     *                      perjd,percal,q,gm,rad,a1,a2,
     *                      bvt,b10n,pyr,darc,comnt3,comnt2
c
20      continue
c
      Else
c
c  .. Process Request for Asteroid Information
      do 30 mx=l1,l2
c                        Asteroids
         read (20,rec=mx) no,astnam,epoch,calepo,equnox,iref,desig,
     1                    rmo,rnode,om,ri,e,Val,
     2                    perjd,percal,q,gm,rad,h,g,bvt,
     3                    rp,albedo,sptype,darc,nobs,comnt1,comnt2
c
c    Convert Double Precision Semi-Major Axis (a) into a string (12 chars wide)
        call fmtdp(val,.true.,sma)
c
c  .. Set All 'NULL' Characters to 'BLANKS'
        astnam(1:1) = ' '
        if(ichar(comnt1(1:1)).eq.0) comnt1 = blank1
        if(ichar(comnt2(1:1)).eq.0) comnt2 = blank2
c
         write(10,'(i6,a,2f11.1,1x,a,1x,a8,1x,a13/4x,4f12.7,f12.9,a/
     *   4x,f15.7,f17.7,f13.9,1X,f7.4,f7.2,f6.2,2f5.2/2x,F9.3,f7.3,a6,
     *   1x,a,i5,a/a)') 
     *                    no,astnam,epoch,calepo,equnox,iref,desig,
     *                    rmo,rnode,om,ri,e,SMa,
     *                    perjd,percal,q,gm,rad,h,g,bvt,
     *                    rp,albedo,sptype,darc,nobs,comnt1,comnt2
c 
30      continue
c
      endif
c
c  .. Request another Range to process
      go to 10
c
c  .. All records have been processed
100     endfile 10
        close(10,status='keep')
        close(20,status='keep')
c
c  .. Clear The Screen
        write(*,'(//////////////////////////)')
      write(*,*) '  The Objects Have Been Written To Editfile'
      write(*,'(//)')
c
      end
      SUBROUTINE FMTDP( VAL, SGNCOL, STR)
      DOUBLE PRECISION  VAL
      LOGICAL           SGNCOL
      CHARACTER*(*)     STR
C
C----------------------------------------------------------------------- 
C  FMTDP (ForMaT Double Precision) converts a double precision value to 
C  a fixed-length character string using a format which adjusts to
C  provide as many significant digits as possible.  The routine uses
C  either F-format or E-format, whichever allows more significant digits 
C  to appear, preferring F-format in case of a tie.  If exponential
C  format is used, one digit appears before the decimal, followed by as 
C  many decimal digits as will fit, the letter 'E', and as compact an
C  exponent as possible (eg, E8 or E-6); the decimal point is suppressed 
C  if the output string is too short otherwise.  The format adjusts to
C  eliminate leading spaces (except possibly one in the first column, if 
C  the calling program reserves column 1 for the sign by setting SGNCOL 
C  to TRUE, and the value is non-negative).  The first digit (or the
C  decimal point) appears in column 2 if SGNCOL is TRUE or the value is 
C  negative; otherwise (if SGNCOL is FALSE and the value is non-
C  negative), it appears in column 1.  If the value is negative, a minus 
C  sign always appears in column 1.  If the output string is too
C  short to accomodate the value under any format, it is filled with
C  asterisks.  If the output string is longer than needed to express the 
C  value with maximum precision, the string is padded with trailing
C  blanks.
C
C  Inputs:
C   VAL     Double precision value to be converted.
C   SGNCOL  Set TRUE to reserve the first column for the sign, ie, to
C           force a space in the first column if the value is positive. 
C           If SGNCOL is FALSE, digits for non-negative values start in 
C           column 1.
C
C  Output:
C   STR     Output string to receive the converted value. 
C----------------------------------------------------------------------- 
C
C-- Parameters:
C   MAXND: Maximum number of digits in a DOUBLE PRECISION value.
C          This should be 16 on a machine with IEEE floating format
C          (such as the IBM PC or Sun), 17 on the VAX (for the default 
C          D-floating format), and 18 on the Unisys.
      INTEGER    MAXND
      PARAMETER (MAXND= 16)
C
C-- Local variables:
      INTEGER    K, P, N, E, J, M, D
      CHARACTER  FMT*10, BUF*24
C
C** Initialize the cursor K and place the sign in the output string.
      K= 1
      IF (VAL .LT. 0.D0) THEN
       STR(1:1)= '-'
       K= 2
      ELSEIF (SGNCOL) THEN
       STR(1:1)= ' '
       K= 2
       ENDIF
C
C** Compute the maximum number of digits P, allowing one column for the 
C   decimal point.  Fill string with asterisks if no room for digits.
      P= LEN( STR ) - K
      IF (P .LE. 0) THEN
       STR= '**'
       RETURN
       ENDIF
C
C** Convert the value to an internal exponential string with P digits.
      D= MIN( P, MAXND) - 1
      WRITE (FMT,'(''(1PD24.'',I2,'')'')') D 
      WRITE (BUF,FMT) VAL
C
C** Get the decimal exponent E.
      E= ( ICHAR( BUF(23:23) ) - ICHAR('0') ) * 10 +
     .     ICHAR( BUF(24:24) ) - ICHAR('0')
      IF (BUF(22:22) .EQ. '-') THEN
       E= -E
      ELSEIF (BUF(22:22) .NE. '+') THEN
       E= ( ICHAR( BUF(22:22) ) - ICHAR('0') ) * 100 + E 
       IF (BUF(21:21) .EQ. '-') E= -E
       ENDIF
C
C** If the exponent is non-negative (absolute values >= 1):
      IF (E .GE. 0) THEN
C
C== Formatting in F-format for values from 1 to 9.999..., or zero: 
C   J is the index in BUF of the first digit.
C   If the value is zero, suppress the zero before the decimal point.
       IF (E .EQ. 0) THEN
        J= 19 - D
        IF (BUF(J:J) .EQ. '0') BUF(J:J+1)= '.0' 
        STR(K:)= BUF(J:20)
        RETURN
C
C== Formatting in F-format for values >= 10:
C   J is the index of the decimal point in BUF.
       ELSEIF (E .LT. MIN( P, MAXND+2)) THEN
        IF (E .GE. MAXND) BUF(21:22)= '00'
        J= 20 - D
        BUF(J:J)= BUF(J-1:J-1)
        STR(K:K+E)= BUF(J:J+E)
        IF (E .LT. D) THEN
         STR(K+E+1:K+E+1)= '.'
         STR(K+E+2:)= BUF(J+E+1:20)
        ELSE
         STR(K+E+1:)= '.'
         ENDIF
        RETURN
C
C== Formatting in E-format:
       ELSE
C
C-- Set M to the minimum number of characters for the exponent.
        M= 4
        IF (E .LE. 99) THEN
         M= 3
         IF (E .LE. 9) M= 2
         ENDIF
C
C-- Compute the number of digits N, allowing for an exponent of M 
C   characters, and ensure N>=0.  N=0 will produce one digit and 
C   suppress the decimal point.
        N= MIN( P-M, MAXND)
        IF (N .GE. 0) THEN
C
C-- If the number of digits must be reduced and rounding up may occur, 
C   reformat the value (ie, let FORTRAN do the rounding).
         IF (N .LT. MAXND) THEN
          J= MAX( N-1, 0)
          IF (BUF(J-D+21:J-D+21) .GE. '5') THEN
           D= J
           WRITE (FMT,'(''(1PD24.'',I2,'')'')') D 
           WRITE (BUF,FMT) VAL
C
C   If the exponent has increased in size, adjust M and N.
           IF (BUF(25-M:25-M) .EQ. '1') THEN
            M= M + 1
            N= N - 1
            ENDIF
           ENDIF
          ENDIF
C
C-- Construct the output string using characters in BUF.
         IF (N .GE. 0) THEN
          STR(K:K+N)= BUF(19-D:20)
          K= K + N
          STR(K+1:K+1)= 'E'
          STR(K+2:)= BUF(26-M:24)
          RETURN
          ENDIF
         ENDIF
        ENDIF
C
C** If the exponent is negative (absolute values < 1):
      ELSE
C
C== Formatting in F-format:
C   J is the index in BUF of the decimal point.
       IF (E .GE. MAX( -P, -4)) THEN
        J= 20 - D
C
C-- If the number of digits must be reduced, and rounding up is needed, 
C   reformat the value directly into the output string.
        D= MIN( P+E, MAXND-1 )
        IF (J+D .LT. 20) THEN
         IF (BUF(J+D+1:J+D+1) .GE. '5') THEN
          D= D - E
          WRITE (FMT,'(''(F'',I2,''.'',I2,'')'')') D+1, D 
          WRITE (STR(K:),FMT) ABS( VAL )
          RETURN
          ENDIF
         ENDIF
C
C-- Otherwise, construct the output string using characters in BUF.
        BUF(J:J)= BUF(J-1:J-1)
        STR(K:K-E-1)= '.000'
        STR(K-E:)= BUF(J:J+D)
        RETURN
C
C== Formatting in E-format:
       ELSE
C
C-- Set M to the minimum number of characters for the exponent.
        M= 5
        IF (E .GE. -99) THEN
         M= 4
         IF (E .GE. -9) M= 3
         ENDIF
C
C-- Compute the number of digits N, allowing for an exponent of M 
C   characters, and ensure N>=0.  N=0 will produce one digit and 
C   suppress the decimal point.
        N= MIN( P-M, MAXND)
        IF (N .GE. 0) THEN
C
C-- If the number of digits must be reduced and rounding up may occur, 
C   reformat the value (ie, let FORTRAN do the rounding).
         IF (N .LT. MAXND) THEN
          J= MAX( N-1, 0)
          IF (BUF(J-D+21:J-D+21) .GE. '5') THEN
           D= J
           WRITE (FMT,'(''(1PD24.'',I2,'')'')') D 
           WRITE (BUF,FMT) VAL
C
C-- If the exponent decreased in size, adjust M and N and add a 0 to the 
C   fraction.
           IF (BUF(27-M:27-M) .LE. '0') THEN
            M= M - 1
            N= N + 1
            BUF(21:21)= '0'
            ENDIF
           ENDIF
          ENDIF
C
C-- Construct the output string using characters in BUF.
         IF (N .GE. 0) THEN
          STR(K:K+N)= BUF(19-D:21)
          K= K + N
          STR(K+1:K+2)= 'E-'
          STR(K+3:)= BUF(27-M:24)
          RETURN
          ENDIF
         ENDIF
        ENDIF
       ENDIF
C
C** The output string is not long enough: fill it with asterisks.
      DO 1 K= 1, LEN(STR)
    1  STR(K:K)= '*'
      END
     
